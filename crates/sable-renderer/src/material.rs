use glam::Vec4;

#[derive(Default)]
pub struct Material {
    pub base_color: Vec4,
    pub metallic: f32,
    pub roughness: f32,
}
