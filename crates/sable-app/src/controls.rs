use std::f32::consts::PI;

use glam::Vec3;
use sable_core::{math::Rad, transform::Transform};

use crate::{MouseButton, input::InputState};

#[derive(Debug, Default, Clone, Copy)]
struct SphericalCoords {
    /// Vertical angle or Zenith. Angle from +Z, in the direction of the reference plane
    pitch: f32,

    /// Horizontal angle or Azimuth.  Angle from +X counter-clockwise
    heading: f32,

    /// Distance from origin
    radius: f32,
}

impl SphericalCoords {
    fn from_vec3(v: Vec3) -> Self {
        let radius = v.length();

        let (pitch, heading) = if radius == 0.0 {
            (0.0, 0.0)
        } else {
            let phi = (v.y / radius).clamp(-1.0, 1.0).acos();
            let theta = v.x.atan2(v.z);
            (phi, theta)
        };

        Self {
            pitch,
            heading,
            radius,
        }
    }

    fn make_safe(&mut self) {
        const EPS: f32 = 0.1;
        self.pitch = self.pitch.clamp(EPS, PI - EPS);
    }
}

pub struct OrbitController {
    pub enable_damping: bool,
    pub damping_factor: f32,
    pub zoom_damping_factor: f32,

    pub enable_zoom: bool,
    pub zoom_speed: f32,

    pub enable_rotate: bool,
    pub rotate_speed: f32,

    pub enable_pan: bool,
    pub pan_speed: f32,

    pub min_distance: f32,
    pub max_distance: f32,
    pub min_vertical_angle: Rad,
    pub max_vertical_angle: Rad,

    // internal state
    coords: SphericalCoords,
    coords_delta: SphericalCoords,
    target: Vec3,
    target_radius: f32,
    pan_offset: Vec3,
}

impl OrbitController {
    pub fn new(position: Vec3, target: Vec3) -> Self {
        let coords = SphericalCoords::from_vec3(position - target);

        Self {
            enable_damping: true,
            damping_factor: 0.05,
            zoom_damping_factor: 0.1,

            enable_zoom: true,
            zoom_speed: 1.0,

            enable_rotate: true,
            rotate_speed: 0.01,

            enable_pan: true,
            pan_speed: 1.0,

            min_distance: 0.0,
            max_distance: f32::INFINITY,
            min_vertical_angle: Rad::new(0.0),
            max_vertical_angle: Rad::new(PI),

            coords,
            coords_delta: SphericalCoords::default(),
            target,
            target_radius: coords.radius,
            pan_offset: Vec3::ZERO,
        }
    }

    pub fn update(&mut self, transform: &mut Transform, input: &InputState, fov: Rad, dt: f32) {
        let screen_height = input.screen_size().y.max(1.0);

        if self.enable_rotate && input.is_mouse_down(MouseButton::Left) {
            let delta = input.mouse_delta();
            let rot_x = 2.0 * PI * delta.x / screen_height;
            let rot_y = 2.0 * PI * delta.y / screen_height;

            self.coords_delta.pitch -= rot_y * self.rotate_speed;
            self.coords_delta.heading -= rot_x * self.rotate_speed;
        }

        let scroll = input.scroll_delta();
        if self.enable_zoom && scroll.y != 0.0 {
            let zoom_scale = 0.95f32.powf(self.zoom_speed);

            if scroll.y > 0.0 {
                self.target_radius *= zoom_scale;
            } else {
                self.target_radius /= zoom_scale;
            }

            self.target_radius = self
                .target_radius
                .clamp(self.min_distance, self.max_distance);
        }

        if self.enable_pan && input.is_mouse_down(MouseButton::Right) {
            let delta = input.mouse_delta();
            let position = transform.position;
            let offset_dir = position - self.target;
            let target_distance = offset_dir.length();
            let target_world_height = 2.0 * target_distance * (fov.as_f32() * 0.5).tan();

            let pan_x = -delta.x * (target_world_height / screen_height) * self.pan_speed;
            let pan_y = delta.y * (target_world_height / screen_height) * self.pan_speed;

            let (right, up, _) = transform.rotation_basis();
            self.pan_offset += right * pan_x + up * pan_y;
        }

        let time_scale = dt; // * 60.0;

        self.coords.pitch += self.coords_delta.pitch * time_scale;
        self.coords.pitch = self.coords.pitch.clamp(
            self.min_vertical_angle.as_f32(),
            self.max_vertical_angle.as_f32(),
        );
        self.coords.heading += self.coords_delta.heading * time_scale;
        self.coords.make_safe();

        if self.enable_damping {
            let lerp_factor = 1.0 - (1.0 - self.zoom_damping_factor).powf(time_scale);
            self.coords.radius += (self.target_radius - self.coords.radius) * lerp_factor
        } else {
            self.coords.radius = self.target_radius
        }
        self.coords.radius = self
            .coords
            .radius
            .clamp(self.min_distance, self.max_distance);

        self.target += self.pan_offset;
        self.pan_offset = Vec3::ZERO;

        // calculate new transform
        let sin_pitch_radius = self.coords.pitch.sin() * self.coords.radius;
        let offset = Vec3::new(
            sin_pitch_radius * self.coords.heading.sin(),
            self.coords.radius * self.coords.pitch.cos(),
            sin_pitch_radius * self.coords.heading.cos(),
        ); // coordinates scaled by radius

        transform.position = self.target + offset;
        transform.look_at(self.target, Vec3::Y);

        // damping decay
        if self.enable_damping {
            let damping = (1.0 - self.damping_factor).powf(time_scale);
            self.coords_delta.pitch *= damping;
            self.coords_delta.heading *= damping;
        } else {
            self.coords_delta.pitch = 0.0;
            self.coords_delta.heading = 0.0;
        }
    }
}
