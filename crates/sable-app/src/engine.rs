use sable_assets::AssetServer;
use sable_renderer::Renderer;
use wgpu::{Device, Queue};

use crate::input::InputState;

/// Orchestration unit for entire workflow
pub struct Engine {
    pub input: InputState,
    pub assets: AssetServer,
    pub renderer: Renderer,
}

impl Engine {
    pub fn new(device: Device, queue: Queue, size: (u32, u32)) -> Self {
        Self {
            input: InputState::new(),
            assets: AssetServer::new(),
            renderer: Renderer::new(device, queue, size),
        }
    }

    pub fn resize(&self, width: u32, height: u32) {
        log::debug!("resize: {width}, {height}");
    }
}
