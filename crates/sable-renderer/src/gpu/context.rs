use wgpu::{Device, Queue};

pub struct WgpuContext {
    pub device: Device,
    pub queue: Queue,
}
