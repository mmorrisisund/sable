use glam::{Affine3A, Mat3, Quat, Vec3};

#[derive(Debug, Clone)]
pub struct Transform {
    pub position: Vec3,
    pub rotation: Quat,
    pub scale: Vec3,

    pub local_matrix: Affine3A,
    pub world_matrix: Affine3A,
    pub prev_world_matrix: Affine3A,

    // Change tracking
    last_position: Vec3,
    last_rotation: Quat,
    last_scale: Vec3,
    force_update: bool,
}

impl Default for Transform {
    fn default() -> Self {
        Self::new()
    }
}

impl Transform {
    pub fn new() -> Self {
        Self {
            position: Vec3::ZERO,
            rotation: Quat::IDENTITY,
            scale: Vec3::ONE,

            local_matrix: Affine3A::IDENTITY,
            world_matrix: Affine3A::IDENTITY,

            last_position: Vec3::ZERO,
            last_rotation: Quat::IDENTITY,
            last_scale: Vec3::ONE,
            prev_world_matrix: Affine3A::IDENTITY,
            force_update: true,
        }
    }

    /// Check if local matrix needs updating.
    ///
    /// Returns true if changed
    pub fn update_local_matrix(&mut self) -> bool {
        let changed = self.position != self.last_position
            || self.rotation != self.last_rotation
            || self.scale != self.last_scale
            || self.force_update;

        if changed {
            self.local_matrix =
                Affine3A::from_scale_rotation_translation(self.scale, self.rotation, self.position);

            self.last_position = self.position;
            self.last_rotation = self.rotation;
            self.last_scale = self.scale;
            self.force_update = false;
        }

        changed
    }

    pub fn look_at(&mut self, target: Vec3, up: Vec3) {
        let forward = (target - self.position).normalize();
        if forward.cross(up).length_squared() < 1e-4 {
            return;
        }

        let right = forward.cross(up).normalize();
        let up = right.cross(forward).normalize();
        let rot_mat = Mat3::from_cols(right, up, -forward);
        self.rotation = Quat::from_mat3(&rot_mat);
    }

    pub fn mark_dirty(&mut self) {
        self.force_update = true;
    }

    /// Local coordinate axes based on current rotation
    ///
    /// Returns (right, up, forward)
    pub fn rotation_basis(&self) -> (Vec3, Vec3, Vec3) {
        let right = self.rotation * Vec3::X;
        let up = self.rotation * Vec3::Y;
        let forward = self.rotation * Vec3::Z;

        (right, up, forward)
    }
}

impl From<gltf::scene::Transform> for Transform {
    fn from(value: gltf::scene::Transform) -> Self {
        let (t, r, s) = value.decomposed();
        let position = t.into();
        let rotation = Quat::from_array(r);
        let scale = s.into();

        Self {
            position,
            rotation,
            scale,
            local_matrix: Affine3A::from_scale_rotation_translation(scale, rotation, position),
            world_matrix: Affine3A::IDENTITY,

            last_position: position,
            last_rotation: rotation,
            last_scale: scale,
            prev_world_matrix: Affine3A::IDENTITY,
            force_update: false,
        }
    }
}
