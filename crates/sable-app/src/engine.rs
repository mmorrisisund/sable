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

    pub fn update(&mut self) {
        // TODO: Update scene

        self.input.start_frame();
    }

    pub fn render_frame(&mut self, view: &TextureView) -> Option<()> {
        let scene = self.scenes.get_active_scene()?;

        let mut frame = self.renderer.begin_frame(scene, &self.assets);

        frame.render(view);

        self.renderer.end_frame();

        Some(())
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.renderer.resize(width, height);

        if let Some(cam) = self
            .scenes
            .get_active_scene_mut()
            .and_then(|s| s.get_active_camera_mut())
        {
            cam.set_aspect(width as f32 / height as f32);
        }
    }
}
