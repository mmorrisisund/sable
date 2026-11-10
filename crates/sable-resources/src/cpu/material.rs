use glam::Vec4;
use sable_core::handle::TextureHandle;

#[derive(Default)]
pub struct Material {
    pub base_color_factor: Vec4,
    pub metallic_factor: f32,
    pub roughness_factor: f32,

    pub base_color_texture: TextureHandle,
}
