use std::{
    borrow::Cow,
    fs::File,
    io::Read,
    path::{Path, PathBuf},
    sync::Arc,
};

use ::image::ImageFormat;
use gltf::{
    Gltf, Node, Primitive,
    buffer::Source,
    mesh::Mode,
    texture::{MagFilter, MinFilter, Sampler, WrappingMode},
};
use sable_core::{
    error::{AssetError, Error, Result},
    handle::{GeometryHandle, ImageHandle, MaterialHandle},
};
use sable_resources::{
    Attribute, AttributeKind, Geometry, Image, IndexAttribute, Material, Mesh,
    image::{ColorSpace, ImageDimension, PixelFormat},
    texture::{Texture, TextureSampler},
};
use wgpu::{
    AddressMode, FilterMode, IndexFormat, MipmapFilterMode, PrimitiveTopology, VertexFormat,
};

use crate::{
    AssetServer,
    prefab::{Prefab, PrefabNode},
};

/// Super basic gltf loader to allow development using the gltf sample assets in increasing complexity
pub struct GltfLoader {
    assets: Arc<AssetServer>,
    buffers: Vec<Vec<u8>>,
    prefab_nodes: Vec<PrefabNode>,
    materials: Vec<MaterialHandle>,

    image_handles: Vec<ImageHandle>,
}

impl GltfLoader {
    pub fn new(assets: Arc<AssetServer>) -> Self {
        Self {
            assets,
            buffers: Vec::new(),
            prefab_nodes: Vec::new(),
            materials: Vec::new(),

            image_handles: Vec::new(),
        }
    }

    pub fn load_model<T: Into<PathBuf>>(&mut self, path: T) -> Result<Prefab> {
        let path = path.into();
        let root = path.parent().unwrap().to_path_buf();

        let file = File::open(path).expect("failed to open model");

        let gltf = gltf::Gltf::from_reader_without_validation(file).unwrap();

        self.load_buffers(&gltf, &root)?;

        self.load_images(&gltf, &root)?;
        // load textures
        self.load_materials(&gltf);

        Ok(self.build_prefab(&gltf))
    }

    fn load_buffers(&mut self, gltf: &Gltf, root: &Path) -> Result<()> {
        let mut data = Vec::new();

        for buffer in gltf.buffers() {
            let blob = gltf.blob.clone();
            match buffer.source() {
                // TODO: Copies blob for every buffer.  Normally one so not a big deal, but can this be optimized to move only applicable data?
                Source::Bin => data.push(blob.unwrap()),
                Source::Uri(name) => {
                    let path = root.join(name);
                    let mut file = File::open(path).unwrap();
                    let mut bytes = Vec::new();
                    file.read_to_end(&mut bytes)?;
                    data.push(bytes);
                }
            }
        }

        self.buffers = data;
        Ok(())
    }

    fn load_images(&mut self, gltf: &Gltf, root: &PathBuf) -> Result<()> {
        use gltf::image;

        for gltf_img in gltf.images() {
            let (bytes, mime_type) = match gltf_img.source() {
                image::Source::View { view, mime_type } => {
                    let start = view.offset();
                    let end = start + view.length();
                    let bytes = self.buffers[view.buffer().index()][start..end].to_vec();

                    (bytes, Some(mime_type))
                }
                image::Source::Uri { uri, mime_type } => {
                    let path = PathBuf::from(root).join(uri);
                    let bytes = load_bytes_from_uri(path)?;
                    (bytes, mime_type)
                }
            };

            let decoded = decode_image_bytes(&bytes, mime_type)?;
            let image = self.assets.image.add(Image {
                data: decoded.data,
                width: decoded.width,
                height: decoded.height,
                format: PixelFormat::Rgba8Unorm,
                dimension: ImageDimension::D2,
            });

            self.image_handles.push(image);
        }

        Ok(())
    }

    fn load_materials(&mut self, gltf: &Gltf) {
        for gltf_mat in gltf.materials() {
            let pbr = gltf_mat.pbr_metallic_roughness();

            let base_color_texture = if let Some(tex_info) = pbr.base_color_texture() {
                let img_handle = self.image_handles[tex_info.texture().source().index()];
                let tex = Texture {
                    name: Some(Cow::Owned("base_color_texture".to_string())),
                    image: img_handle,
                    color_space: ColorSpace::Srgb,
                    sampler: gltf_to_engine_sampler(tex_info.texture().sampler()),
                };
                self.assets.texture.add(tex)
            } else {
                self.assets.default_white
            };

            let handle = self.assets.material.add(Material {
                base_color_factor: pbr.base_color_factor().into(),
                metallic_factor: pbr.metallic_factor(),
                roughness_factor: pbr.roughness_factor(),
                base_color_texture,
            });

            self.materials.push(handle);
        }
    }

    fn build_prefab(&mut self, gltf: &Gltf) -> Prefab {
        for node in gltf.nodes() {
            self.prefab_nodes.push(prefab_node_from_gltf(&node));
        }

        for node in gltf.nodes() {
            self.bind_node_mesh(&node);
        }

        let root_indices =
            if let Some(scene) = gltf.default_scene().or_else(|| gltf.scenes().next()) {
                scene.nodes().map(|n| n.index()).collect()
            } else {
                Vec::new()
            };

        Prefab {
            nodes: std::mem::take(&mut self.prefab_nodes),
            root_indices,
        }
    }

    fn bind_node_mesh(&mut self, node: &Node) {
        if let Some(gltf_mesh) = node.mesh() {
            let primitives = gltf_mesh.primitives().collect::<Vec<_>>();

            match primitives.len() {
                0 => {}
                1 => {
                    let mesh = self.build_mesh(&primitives[0]);
                    self.prefab_nodes[node.index()].mesh = Some(mesh);
                }
                _ => panic!("split primitives not handled yet"),
            }
        }
    }

    fn build_mesh(&mut self, primitive: &Primitive) -> Mesh {
        let geometry = self.build_geometry(primitive);

        let material = self.materials[primitive.material().index().unwrap()];

        Mesh { geometry, material }
    }

    fn build_geometry(&mut self, primitive: &Primitive) -> GeometryHandle {
        let mut geometry = Geometry::default();

        let Some(pos_accessor) = primitive.get(&gltf::Semantic::Positions) else {
            return self.assets.geometry.add(geometry);
        };

        let vertex_count = pos_accessor.count();
        if vertex_count == 0 {
            return self.assets.geometry.add(geometry);
        }
        geometry.vertex_count = vertex_count;

        let reader = primitive.reader(|buffer| Some(&self.buffers[buffer.index()]));

        let positions = reader
            .read_positions()
            .map(std::iter::Iterator::collect::<Vec<_>>)
            .unwrap_or_default();
        geometry.set_attribute(
            Attribute::new(&positions, VertexFormat::Float32x3),
            AttributeKind::Position,
        );

        let indices = reader
            .read_indices()
            .map(|i| i.into_u32())
            .map(std::iter::Iterator::collect::<Vec<_>>);
        if let Some(data) = indices {
            geometry.set_index_attribute(Some(IndexAttribute::new(&data, IndexFormat::Uint32)));
        }

        // Normals, UV, Tangents, etc.
        self.load_surface_attributes(primitive, &mut geometry);

        geometry.topology = match primitive.mode() {
            Mode::Points => PrimitiveTopology::PointList,
            Mode::LineStrip => PrimitiveTopology::LineStrip,
            Mode::Lines | Mode::LineLoop => PrimitiveTopology::LineList,
            Mode::TriangleStrip => PrimitiveTopology::TriangleStrip,
            Mode::Triangles | Mode::TriangleFan => PrimitiveTopology::TriangleList,
        };

        self.assets.geometry.add(geometry.clone())
    }

    fn load_surface_attributes(&mut self, primitive: &Primitive, geometry: &mut Geometry) {
        let reader = primitive.reader(|buffer| Some(&self.buffers[buffer.index()]));

        let colors = if let Some(colors) = reader.read_colors(0) {
            colors.into_rgb_f32().collect::<Vec<[f32; 3]>>()
        } else {
            vec![[1.0; 3]; geometry.vertex_count]
        };
        geometry.set_attribute(
            Attribute::new(&colors, VertexFormat::Float32x3),
            AttributeKind::Color,
        );

        let normals = if let Some(normals) = reader.read_normals() {
            normals.collect()
        } else {
            vec![[0.0; 3]]
        };
        geometry.set_attribute(
            Attribute::new(&normals, VertexFormat::Float32x3),
            AttributeKind::Normal,
        );

        let coords = if let Some(coords) = reader.read_tex_coords(0) {
            coords.into_f32().collect()
        } else {
            vec![[0.0; 2]]
        };
        geometry.set_attribute(
            Attribute::new(&coords, VertexFormat::Float32x2),
            AttributeKind::TexCoords,
        );
    }
}

// TODO: can(should?) this be async
fn load_bytes_from_uri(path: PathBuf) -> Result<Vec<u8>> {
    // TODO: Handle data uris
    let mut bytes = Vec::new();

    let mut f = std::fs::File::open(path)?;
    f.read_to_end(&mut bytes)?;

    Ok(bytes)
}

/// Intermediate data structure to image data before saving to the asset server
struct DecodedImage {
    data: Vec<u8>,
    width: u32,
    height: u32,
}

/// Used to decode raw bytes into specific image
fn decode_image_bytes(bytes: &[u8], mime_type: Option<&str>) -> Result<DecodedImage> {
    let res = if let Some(mime_type) = mime_type {
        match ImageFormat::from_mime_type(mime_type) {
            Some(format) => ::image::load_from_memory_with_format(bytes, format),
            None => ::image::load_from_memory(bytes),
        }
    } else {
        ::image::load_from_memory(bytes)
    };

    match res {
        Ok(img) => {
            let rgba = img.into_rgba8();

            Ok(DecodedImage {
                data: rgba.to_vec(),
                width: rgba.width(),
                height: rgba.height(),
            })
        }
        Err(e) => Err(Error::Asset(AssetError::from(e))),
    }
}

//------------------------------
// gltf helpers
//------------------------------

fn prefab_node_from_gltf(node: &Node) -> PrefabNode {
    PrefabNode {
        name: node.name().map(str::to_owned),
        mesh: None,
        children: node.children().map(|n| n.index()).collect(),
        transform: node.transform().into(),
    }
}

fn gltf_to_engine_sampler(value: Sampler) -> TextureSampler {
    TextureSampler {
        address_mode_u: wrapping_mode(value.wrap_s()),
        address_mode_v: wrapping_mode(value.wrap_t()),
        address_mode_w: AddressMode::ClampToEdge,
        mag_filter: mag_filter_mode(value.mag_filter()),
        min_filter: min_filter_mode(value.min_filter()),
        mipmap_filter: mipmap_filter_mode(value.min_filter()),
    }
}

fn wrapping_mode(mode: WrappingMode) -> AddressMode {
    match mode {
        WrappingMode::ClampToEdge => AddressMode::ClampToEdge,
        WrappingMode::MirroredRepeat => AddressMode::MirrorRepeat,
        WrappingMode::Repeat => AddressMode::Repeat,
    }
}

fn mag_filter_mode(mode: Option<MagFilter>) -> FilterMode {
    if let Some(mode) = mode {
        match mode {
            MagFilter::Nearest => FilterMode::Nearest,
            MagFilter::Linear => FilterMode::Linear,
        }
    } else {
        FilterMode::Nearest
    }
}

fn min_filter_mode(mode: Option<MinFilter>) -> FilterMode {
    use MinFilter::*;

    if let Some(mode) = mode {
        match mode {
            Nearest | NearestMipmapLinear | NearestMipmapNearest => FilterMode::Nearest,
            Linear | LinearMipmapLinear | LinearMipmapNearest => FilterMode::Linear,
        }
    } else {
        FilterMode::Nearest
    }
}

fn mipmap_filter_mode(mode: Option<MinFilter>) -> MipmapFilterMode {
    use MinFilter::*;

    if let Some(mode) = mode {
        match mode {
            Nearest | NearestMipmapNearest | LinearMipmapNearest => MipmapFilterMode::Nearest,
            Linear | NearestMipmapLinear | LinearMipmapLinear => MipmapFilterMode::Linear,
        }
    } else {
        MipmapFilterMode::Nearest
    }
}
