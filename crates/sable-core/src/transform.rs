use glam::{Mat4, Quat, Vec3};

#[derive(Debug, Default, Clone)]
pub struct Transform {
    translate: Vec3,
    rotate: Quat,
    scale: Vec3,

    local_matrix: Mat4,
    world_matrix: Mat4,

    dirty: bool,
}

impl Transform {
    pub fn new() -> Self {
        Self {
            translate: Vec3::ZERO,
            rotate: Quat::IDENTITY,
            scale: Vec3::ONE,

            local_matrix: Mat4::IDENTITY,
            world_matrix: Mat4::IDENTITY,

            dirty: true,
        }
    }

    pub fn mark_dirty(&mut self) {
        self.dirty = true;
    }

    pub fn set_translate(&mut self, translate: Vec3) {
        self.translate = translate;
        self.dirty = true;
    }

    pub fn set_rotate(&mut self, rotate: Quat) {
        self.rotate = rotate;
        self.dirty = true;
    }

    pub fn set_scale(&mut self, scale: Vec3) {
        self.scale = scale;
        self.dirty = true;
    }

    pub fn with_translation(mut self, translate: Vec3) -> Self {
        self.translate = translate;
        self.dirty = true;
        self
    }

    pub fn with_rotation(mut self, rotate: Quat) -> Self {
        self.rotate = rotate;
        self.dirty = true;
        self
    }

    pub fn with_scale(mut self, scale: Vec3) -> Self {
        self.scale = scale;
        self.dirty = true;
        self
    }

    pub fn set_local_matrix(&mut self, matrix: Mat4) {
        self.local_matrix = matrix;
    }

    pub fn set_world_matrix(&mut self, matrix: Mat4) {
        self.world_matrix = matrix;
    }

    pub fn local_matrix(&self) -> Mat4 {
        self.local_matrix
    }

    pub fn world_matrix(&self) -> Mat4 {
        self.world_matrix
    }

    pub fn matrix(&mut self) -> Mat4 {
        self.update();
        self.local_matrix
    }

    pub fn update(&mut self) -> bool {
        let mut changed = false;

        if self.dirty {
            self.local_matrix =
                Mat4::from_scale_rotation_translation(self.scale, self.rotate, self.translate);
            self.dirty = false;
            changed = true;
        }

        changed
    }
}

impl From<gltf::scene::Transform> for Transform {
    fn from(value: gltf::scene::Transform) -> Self {
        let (t, r, s) = value.decomposed();

        Self::new()
            .with_rotation(Quat::from_array(r))
            .with_translation(t.into())
            .with_scale(s.into())
    }
}
