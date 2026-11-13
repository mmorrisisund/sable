use std::{path::PathBuf, sync::Arc};

use sable_core::{
    error::Result,
    handle::{GeometryHandle, ImageHandle, MaterialHandle, PrefabHandle, TextureHandle},
};
use sable_resources::{Geometry, Image, image::ColorSpace, material::Material, texture::Texture};

use crate::{container::Container, loader::GltfLoader, prefab::Prefab};

#[derive(Clone)]
pub struct AssetServer {
    pub prefab: Arc<Container<PrefabHandle, Prefab>>,
    pub geometry: Arc<Container<GeometryHandle, Geometry>>,
    pub material: Arc<Container<MaterialHandle, Material>>,
    pub image: Arc<Container<ImageHandle, Image>>,
    pub texture: Arc<Container<TextureHandle, Texture>>,

    pub default_white: TextureHandle,
    pub default_black: TextureHandle,
    pub default_normal: TextureHandle,
}

impl AssetServer {
    pub fn new() -> Self {
        let image = Arc::new(Container::new());
        let texture = Arc::new(Container::new());

        let white_img = image.add(Image::solid_color([255; 4]));
        let mut default_white = Texture::new_2d(Some("default_white"), white_img);
        default_white.color_space = ColorSpace::Srgb;
        let default_white = texture.add(default_white);

        let black_img = image.add(Image::solid_color([0, 0, 0, 255]));
        let mut black_tex = Texture::new_2d(Some("default_black"), black_img);
        black_tex.color_space = ColorSpace::Srgb;
        let default_black = texture.add(black_tex);

        let normal_img = image.add(Image::solid_color([128, 128, 255, 255]));
        let mut normal_tex = Texture::new_2d(Some("default_normal"), normal_img);
        normal_tex.color_space = ColorSpace::Srgb;
        let default_normal = texture.add(normal_tex);

        Self {
            prefab: Arc::new(Container::new()),
            geometry: Arc::new(Container::new()),
            material: Arc::new(Container::new()),
            image,
            texture,
            default_white,
            default_black,
            default_normal,
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
