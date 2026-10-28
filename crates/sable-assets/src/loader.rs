use std::{fs::File, io::Read, path::PathBuf, sync::Arc};

use gltf::{Gltf, Node, Primitive, buffer::Source, mesh::Mode};
use sable_core::{error::Result, handle::GeometryHandle};
use sable_resources::{Attribute, AttributeKind, Geometry, IndexAttribute, Mesh};
use wgpu::{IndexFormat, PrimitiveTopology, VertexFormat};

use crate::{
    AssetServer,
    prefab::{Prefab, PrefabNode},
};

/// Super basic gltf loader to allow development using the gltf sample assets in increasing complexity
pub struct GltfLoader {
    assets: Arc<AssetServer>,
    buffers: Vec<Vec<u8>>,
    prefab_nodes: Vec<PrefabNode>,
}

impl GltfLoader {
    pub fn new(assets: Arc<AssetServer>) -> Self {
        Self {
            assets,
            buffers: Vec::new(),
            prefab_nodes: Vec::new(),
        }
    }

    pub fn load_model<T: Into<PathBuf>>(&mut self, path: T) -> Result<Prefab> {
        let path = path.into();
        let root = path.parent().unwrap().to_path_buf();

        let file = File::open(path).expect("failed to open model");

        let gltf = gltf::Gltf::from_reader_without_validation(file).unwrap();

        self.load_buffers(&gltf, root);

        // load images
        // load textures
        // load materials

        Ok(self.build_prefab(&gltf))
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

        // find and attach material
        Mesh { geometry }
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
    }
}

fn prefab_node_from_gltf(node: &Node) -> PrefabNode {
    PrefabNode {
        name: node.name().map(str::to_owned),
        mesh: None,
        children: node.children().map(|n| n.index()).collect(),
    }
}
