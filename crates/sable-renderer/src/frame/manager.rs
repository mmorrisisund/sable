use sable_assets::AssetServer;
use sable_resources::Scene;

use crate::{
    core::scene::RenderableScene, frame::state::FrameState, gpu::manager::ResourceManager,
};

pub struct FrameManager {
    pub scene: RenderableScene,
    frame_state: FrameState,
}

impl FrameManager {
    pub fn new() -> Self {
        Self {
            scene: RenderableScene::default(),
            frame_state: FrameState::default(),
        }
    }

    /// Get renderable items from scene
    /// and update frame global data for shaders
    pub fn extract_scene(
        &mut self,
        scene: &Scene,
        assets: &AssetServer,
        resources: &mut ResourceManager,
    ) -> Option<()> {
        // extract renderable scene from cpu scene
        self.scene.prepare(scene, assets, resources);

        let camera = &scene.get_active_camera()?.as_render_camera();
        self.frame_state.update(camera);

        resources.prepare_frame_binding(&self.frame_state);

        Some(())
    }
}
