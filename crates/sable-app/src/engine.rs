use gpui::WgpuSurfaceHandle;
use sable_assets::AssetServer;
use sable_renderer::Renderer;
use sable_resources::SceneManager;
use wgpu::TextureView;

use crate::input::InputState;

/// Orchestration unit for entire workflow
pub struct Engine {
    pub input: InputState,
    pub assets: AssetServer,
    pub scenes: SceneManager,
    pub renderer: Renderer,
}

impl Engine {
    pub fn new(surface: WgpuSurfaceHandle) -> Self {
        Self {
            input: InputState::new(),
            assets: AssetServer::new(),
            scenes: SceneManager::new(),
            renderer: Renderer::new(surface),
        }
    }

    pub fn render_frame(&mut self, view: &TextureView) -> Option<()> {
        let scene = self.scenes.get_active_scene()?;
        let camera = scene.get_active_camera()?;

        self.renderer.begin_frame(scene, &self.assets);
        // frame.render(view);
        self.renderer.prepare_camera();
        self.renderer.render(camera, view);
        self.renderer.end_frame();

        Some(())
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.renderer.resize(width, height);
    }
}
