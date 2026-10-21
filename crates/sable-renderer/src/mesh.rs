use bytemuck::{Pod, Zeroable};
use wgpu::{VertexAttribute, VertexBufferLayout, vertex_attr_array};

pub struct Mesh {
    pub(crate) vertices: Vec<Vertex>,
}

impl Mesh {
    pub fn triangle() -> Mesh {
        let vertices = vec![
            Vertex {
                position: [-1.0f32, -1.0, 0.0],
                color: [1.0, 0.0, 0.0],
            },
            Vertex {
                position: [0.0, 1.0, 0.0],
                color: [0.0, 1.0, 0.0],
            },
            Vertex {
                position: [1.0, -1.0, 0.0],
                color: [0.0, 0.0, 1.0],
            },
        ];

        Mesh { vertices }
    }
}

#[repr(C)]
#[derive(Debug, Default, Clone, Copy, Pod, Zeroable)]
pub struct Vertex {
    position: [f32; 3],
    color: [f32; 3],
}

impl Vertex {
    const ATTRS: [VertexAttribute; 2] = vertex_attr_array![
        0 => Float32x3,
        1 => Float32x3
    ];

    pub(crate) fn layout() -> VertexBufferLayout<'static> {
        VertexBufferLayout {
            array_stride: size_of::<Vertex>() as u64,
            attributes: &Self::ATTRS,
            step_mode: wgpu::VertexStepMode::Vertex,
        }
    }
}
