use sable_assets::AssetServer;
use sable_resources::Scene;

use crate::{
    core::scene::{RenderableItem, RenderableScene},
    gpu::manager::ResourceManager,
};

pub struct FrameManager {
    pub scene: RenderableScene,
}

impl FrameManager {
    pub fn new() -> Self {
        Self {
            scene: RenderableScene::default(),
        }
    }

    pub fn extract_scene(
        &mut self,
        scene: &Scene,
        assets: &AssetServer,
        resources: &mut ResourceManager,
        // ) -> RenderableScene {
    ) -> Vec<RenderableItem> {
        // extract renderable scene from cpu scene
        self.scene.extract(scene, assets, resources);
        self.scene.items.clone()
        // update frame state (a wrapper around uniform buffer or data)

        // upload that frame state (prepare globals) FrameGlobals holdings binding info
        // bg, bgl, wgsl
    }
}
