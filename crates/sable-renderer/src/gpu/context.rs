use gpui::WgpuSurfaceHandle;
use wgpu::{Device, Queue, TextureFormat};

pub struct WgpuContext {
    pub device: Device,
    pub queue: Queue,

    pub depth_format: TextureFormat,
    pub surface_format: TextureFormat,
}

impl WgpuContext {
    pub fn from_surface_handle(surface: &WgpuSurfaceHandle) -> Self {
        Self {
            device: surface.device().clone(),
            queue: surface.queue().clone(),
            surface_format: surface.format(),
            depth_format: TextureFormat::Depth32Float, // TODO: Take from InitialConfig
        }
    }
}
