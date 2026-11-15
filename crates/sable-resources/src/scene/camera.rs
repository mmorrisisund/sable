use bytemuck::{Pod, Zeroable};
use glam::{Affine3A, Mat4, Vec3};
use sable_core::math::Rad;

#[derive(Debug, Default)]
pub struct Camera {
    projection: ProjectionType,
    fov: Rad,
    aspect: f32,
    near: f32,
    far: f32,

    view_matrix: Mat4,
    world_matrix: Affine3A,

    projection_matrix: Mat4,
    view_projection_matrix: Mat4,
}

#[derive(Debug, Default)]
pub enum ProjectionType {
    #[default]
    Perspective,
    Orthographic,
}

impl Camera {
    pub fn new_perspective(fov: impl Into<Rad>, aspect: f32, near: f32, far: f32) -> Self {
        Self {
            fov: fov.into(),
            projection: ProjectionType::Perspective,
            aspect,
            near,
            far,

            view_matrix: Mat4::IDENTITY,
            world_matrix: Affine3A::IDENTITY,

            projection_matrix: Mat4::IDENTITY,
            view_projection_matrix: Mat4::IDENTITY,
        }
    }

    pub fn new_orthographic() -> Self {
        // Temporary placeholder
        Self {
            projection: ProjectionType::Orthographic,
            ..Default::default()
        }
    }
}

pub trait RenderCamera {
    fn extract_uniform(&self) -> CameraUniform;
}

#[repr(C)]
#[derive(Default, Clone, Copy, Pod, Zeroable)]
pub struct CameraUniform {
    pub view: Mat4,
    pub projection: Mat4,
    pub view_projection: Mat4,
    pub position: Vec3,
    pub _pad0: f32,
}

impl RenderCamera for Camera {
    fn extract_uniform(&self) -> CameraUniform {
        CameraUniform {
            view: self.view_matrix,
            projection: self.projection_matrix,
            view_projection: self.view_projection_matrix,
            position: self.world_matrix.translation.into(),
            ..Default::default()
        }
    }
}
