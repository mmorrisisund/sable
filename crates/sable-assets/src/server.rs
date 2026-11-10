use std::{path::PathBuf, sync::Arc};

use sable_core::{
    error::Result,
    handle::{GeometryHandle, ImageHandle, MaterialHandle, PrefabHandle, TextureHandle},
};
use sable_resources::{Geometry, Image, material::Material, texture::Texture};

use crate::{container::Container, loader::GltfLoader, prefab::Prefab};

#[derive(Clone)]
pub struct AssetServer {
    pub prefab: Arc<Container<PrefabHandle, Prefab>>,
    pub geometry: Arc<Container<GeometryHandle, Geometry>>,
    pub material: Arc<Container<MaterialHandle, Material>>,
    pub image: Arc<Container<ImageHandle, Image>>,
    pub texture: Arc<Container<TextureHandle, Texture>>,
}

impl AssetServer {
    pub fn new() -> Self {
        Self {
            prefab: Arc::new(Container::new()),
            geometry: Arc::new(Container::new()),
            material: Arc::new(Container::new()),
            image: Arc::new(Container::new()),
            texture: Arc::new(Container::new()),
        }
    }
    pub fn load_model<T: Into<PathBuf>>(&mut self, path: T) -> Result<PrefabHandle> {
        let mut loader = GltfLoader::new(Arc::new(self.clone()));
        let prefab = loader.load_model(path)?;
        let handle = self.prefab.add(prefab);

        Ok(handle)
    }
}

impl Default for AssetServer {
    fn default() -> Self {
        Self::new()
    }
}
