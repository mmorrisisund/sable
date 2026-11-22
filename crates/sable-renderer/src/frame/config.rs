use wgpu::TextureFormat;

pub struct FrameConfig {
    /// screen width
    pub width: u32,
    /// screen height
    pub height: u32,
    pub depth_format: TextureFormat,
    pub surface_format: TextureFormat,
}
