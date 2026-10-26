use std::path::PathBuf;

use slotmap::SlotMap;

use sable_core::{error::Result, handle::GeometryHandle};
use sable_resources::{Geometry, Mesh};

use crate::loader::GltfLoader;

pub struct AssetServer {
    pub geometry: SlotMap<GeometryHandle, Geometry>,
}

impl AssetServer {
    pub fn new() -> Self {
        Self {
            geometry: SlotMap::default(),
        }
    }
    pub fn load_model<T: Into<PathBuf>>(&mut self, path: T) -> Result<Mesh> {
        let loader = GltfLoader;
        Ok(loader.load_model(path))
    }
}

impl Default for AssetServer {
    fn default() -> Self {
        Self::new()
    }
}
