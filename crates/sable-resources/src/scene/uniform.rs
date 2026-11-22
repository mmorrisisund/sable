use bytemuck::{Pod, Zeroable};
use glam::{Mat4, Vec3};

#[repr(C)]
#[derive(Default, Clone, Copy, Pod, Zeroable)]
pub struct FrameUniform {
    pub view: Mat4,
    pub projection: Mat4,
    pub view_projection: Mat4,
    pub position: Vec3,
    pub _pad0: f32,
}
