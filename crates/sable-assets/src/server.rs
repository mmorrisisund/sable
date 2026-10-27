use std::{path::PathBuf, sync::Arc};

use sable_core::{error::Result, handle::GeometryHandle};
use sable_resources::{Geometry, Mesh};

use crate::{container::Container, loader::GltfLoader};

pub struct AssetServer {
    pub geometry: Arc<Container<GeometryHandle, Geometry>>,
}

impl AssetServer {
    pub fn new() -> Self {
        Self {
            geometry: Arc::new(Container::new()),
        }
    }
    pub fn load_model<T: Into<PathBuf>>(&mut self, path: T) -> Result<Mesh> {
        let mut loader = GltfLoader::new();
        loader.load_model(path, self)
    }
}

impl Default for AssetServer {
    fn default() -> Self {
        Self::new()
    }
}
