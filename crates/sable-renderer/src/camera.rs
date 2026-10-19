use glam::{Mat4, Vec3};

#[derive(Default)]
pub struct Camera {
    view: Mat4,
    projection: Mat4,
    position: Vec3,
}

impl Camera {
    pub fn perspective(
        position: Vec3,
        target: Vec3,
        fov_y_radians: f32,
        aspect: f32,
        near: f32,
        far: f32,
    ) -> Camera {
        let view = glam::camera::rh::view::look_at_mat4(position, target, Vec3::Y);
        let proj = glam::camera::rh::proj::directx::perspective(fov_y_radians, aspect, near, far);

        Camera {
            view,
            projection: proj,
            position,
        }
    }
}
