use std::{fs::File, io::Read, path::PathBuf};

use gltf::{Gltf, buffer::Source};
use wgpu::VertexFormat;

use crate::{
    Mesh,
    mesh::{Attribute, AttributeKind},
};

/// Super basic gltf loader to allow development using the gltf sample assets in increasing complexity
pub struct GltfLoader;

impl GltfLoader {
    pub fn load_model<T: Into<PathBuf>>(name: T) -> Mesh {
        let path = name.into();
        let root = path.parent().unwrap().to_path_buf();

        let file = File::open(path).expect("failed to open model");

        let gltf = gltf::Gltf::from_reader_without_validation(file).unwrap();

        let buffers = get_buffers(&gltf, root);

        let gltf_mesh = gltf.meshes().next().unwrap();
        let mut mesh = Mesh::default();

        for primitive in gltf_mesh.primitives() {
            let reader = primitive.reader(|buffer| Some(&buffers[buffer.index()]));
            let positions = reader.read_positions().unwrap().collect::<Vec<[f32; 3]>>();
            mesh.geometry.set_attribute(
                Attribute::new(&positions, VertexFormat::Float32x3),
                AttributeKind::Position,
            );

            let colors = if let Some(colors) = reader.read_colors(0) {
                colors.into_rgb_f32().collect::<Vec<[f32; 3]>>()
            } else {
                vec![[1.0; 3]; positions.len()]
            };
            mesh.geometry.set_attribute(
                Attribute::new(&colors, VertexFormat::Float32x3),
                AttributeKind::Color,
            );
        }

        mesh
    }
}

fn get_buffers(gltf: &Gltf, root: PathBuf) -> Vec<Vec<u8>> {
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

    data
}
