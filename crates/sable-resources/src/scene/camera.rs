use glam::{Affine3A, Mat4, Vec3, camera::rh::proj};
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
        let mut cam = Self {
            fov: fov.into(),
            projection: ProjectionType::Perspective,
            aspect,
            near,
            far,

            view_matrix: Mat4::IDENTITY,
            world_matrix: Affine3A::IDENTITY,

            projection_matrix: Mat4::IDENTITY,
            view_projection_matrix: Mat4::IDENTITY,
        };

        cam.update_projection();

        cam
    }

    pub fn new_orthographic() -> Self {
        // Temporary placeholder
        Self {
            projection: ProjectionType::Orthographic,
            ..Default::default()
        }
    }

    pub fn update_projection(&mut self) {
        self.projection_matrix = match self.projection {
            ProjectionType::Perspective => {
                proj::directx::perspective(self.fov.as_f32(), self.aspect, self.near, self.far)
            }
            ProjectionType::Orthographic => todo!(),
        };

        self.view_projection_matrix = self.projection_matrix * self.view_matrix;
    }

    pub fn update_view_projection(&mut self, world_matrix: &Affine3A) {
        let world_matrix = *world_matrix;
        self.view_matrix = Mat4::from(world_matrix).inverse();
        self.view_projection_matrix = self.projection_matrix * self.view_matrix;
        self.world_matrix = world_matrix;
    }

    pub fn fov(&self) -> Rad {
        self.fov
    }

    pub fn as_render_camera(&self) -> RenderCamera {
        RenderCamera {
            view: self.view_matrix,
            projection: self.projection_matrix,
            view_projection: self.view_projection_matrix,
            position: self.world_matrix.translation.into(),
        }
    }
}

/// Lightweight version of camera to use in renderer
#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct RenderCamera {
    pub view: Mat4,
    pub projection: Mat4,
    pub view_projection: Mat4,
    pub position: Vec3,
}
