use std::{fs::File, io::Read, path::PathBuf};

use gltf::{Gltf, Node, Primitive, buffer::Source, mesh::Mode};
use sable_core::error::Result;
use sable_resources::{Attribute, AttributeKind, Geometry, IndexAttribute, Mesh};
use wgpu::{IndexFormat, PrimitiveTopology, VertexFormat};

use crate::AssetServer;

/// Super basic gltf loader to allow development using the gltf sample assets in increasing complexity
pub struct GltfLoader {
    buffers: Vec<Vec<u8>>,
}

impl GltfLoader {
    pub fn new() -> Self {
        Self {
            buffers: Vec::new(),
        }
    }
    pub fn load_model<T: Into<PathBuf>>(&mut self, path: T, assets: &AssetServer) -> Result<Mesh> {
        let path = path.into();
        let root = path.parent().unwrap().to_path_buf();

        let file = File::open(path).expect("failed to open model");

        let gltf = gltf::Gltf::from_reader_without_validation(file).unwrap();

        self.load_buffers(&gltf, root);

        // load images
        // load textures
        // load materials
        // build prefab here, move this into that.
        let node = gltf.nodes().next().unwrap();

        Ok(self.build_node_mesh(&node, assets))
    }

    fn load_buffers(&mut self, gltf: &Gltf, root: PathBuf) {
        let mut data = Vec::new();
        // TODO: for each buffer determine source.  If bin then use blob, else load from uri path

        for buffer in gltf.buffers() {
            let blob = gltf.blob.clone();
            match buffer.source() {
                Source::Bin => data.push(blob.unwrap()),
                Source::Uri(name) => {
                    let path = root.join(name);
                    log::debug!("{path:?}");
                    let mut file = File::open(path).unwrap();
                    let mut bytes = Vec::new();
                    let _ = file.read_to_end(&mut bytes);
                    data.push(bytes);
                }
            }
        }

        self.buffers = data;
    }

    fn build_node_mesh(&mut self, node: &Node, assets: &AssetServer) -> Mesh {
        if let Some(mesh) = node.mesh() {
            let primitives = mesh.primitives().collect::<Vec<_>>();

            match primitives.len() {
                0 => {
                    panic!("1 primitive required ")
                } // eventually this can be a no-op
                1 => {
                    let geometry = self.build_geometry(&primitives[0], assets);
                    let geometry_handle = assets.geometry.add(geometry.clone());
                    return Mesh {
                        geometry,
                        geometry_handle,
                    };
                }
                _ => panic!("split primitives not handled yet"),
            }
        }
        panic!("mesh not found")
        // find and attach material
    }

    fn build_geometry(&mut self, primitive: &Primitive, _assets: &AssetServer) -> Geometry {
        let mut geometry = Geometry::default();

        let Some(pos_accessor) = primitive.get(&gltf::Semantic::Positions) else {
            return geometry;
        };

        let vertex_count = pos_accessor.count();
        if vertex_count == 0 {
            return geometry;
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

        // geometry_handle = assets.geometry.add(geometry);
        geometry
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
    }
}
