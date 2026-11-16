use wgpu::TextureFormat;

pub struct FrameConfig {
    pub width: u32,
    pub height: u32,
    pub depth_format: TextureFormat,
    pub surface_format: TextureFormat,
}
