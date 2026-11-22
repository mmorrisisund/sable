//! Frame binding state
//! Contains raw data to pass to the frame global binding (group 0)

use bytemuck::{Pod, Zeroable};
use glam::{Mat4, Vec3};
use sable_resources::RenderCamera;

#[repr(C)]
#[derive(Default, Clone, Copy, Pod, Zeroable)]
pub struct FrameState {
    pub view: Mat4,
    pub projection: Mat4,
    pub view_projection: Mat4,
    pub position: Vec3,
    pub dt: f32,
}

impl FrameState {
    pub fn update(&mut self, camera: &RenderCamera) {
        self.view = camera.view;
        self.projection = camera.projection;
        self.view_projection = camera.view_projection;
        self.position = camera.position;
    }
}
