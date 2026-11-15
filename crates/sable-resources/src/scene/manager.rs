use sable_core::handle::SceneHandle;
use slotmap::SlotMap;

use crate::scene::Scene;

#[derive(Default)]
pub struct SceneManager {
    scenes: SlotMap<SceneHandle, Scene>,
    active_scene: Option<SceneHandle>,
}

impl SceneManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_scene(&mut self, scene: Scene) -> SceneHandle {
        self.scenes.insert(scene)
    }

    pub fn set_active_scene(&mut self, scene: SceneHandle) {
        self.active_scene = Some(scene);
    }

    pub fn add_active_scene(&mut self, scene: Scene) -> SceneHandle {
        let handle = self.add_scene(scene);
        self.set_active_scene(handle);
        handle
    }

    pub fn get_active_scene(&self) -> Option<&Scene> {
        self.active_scene.and_then(|h| self.scenes.get(h))
    }

    pub fn get_active_scene_mut(&mut self) -> Option<&mut Scene> {
        self.active_scene.and_then(|h| self.scenes.get_mut(h))
    }

    pub fn query_camera_bundle_mut(&mut self) {}
}
