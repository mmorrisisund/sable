use sable_assets::AssetServer;
use sable_renderer::Renderer;
use sable_resources::SceneManager;
use wgpu::{Device, Queue, TextureView};

use crate::input::InputState;

/// Orchestration unit for entire workflow
pub struct Engine {
    pub input: InputState,
    pub assets: AssetServer,
    pub scenes: SceneManager,
    pub renderer: Renderer,
}

impl Engine {
    pub fn new(device: Device, queue: Queue, size: (u32, u32)) -> Self {
        Self {
            input: InputState::new(),
            assets: AssetServer::new(),
            scenes: SceneManager::new(),
            renderer: Renderer::new(device, queue, size),
        }
    }

    pub fn render_frame(&mut self, view: &TextureView) {
        let Some(scene) = self.scenes.get_active_scene() else {
            return;
        };
        let Some(active_cam) = scene.get_active_camera() else {
            return;
        };

        self.renderer.render(active_cam, view);
    }

    pub fn resize(&self, width: u32, height: u32) {
        log::debug!("resize: {width}, {height}");
    }
}
