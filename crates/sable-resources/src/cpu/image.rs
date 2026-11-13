use wgpu::{TextureDimension, TextureFormat, TextureViewDimension};

pub struct Image {
    pub data: Vec<u8>,
    pub width: u32,
    pub height: u32,
    pub format: PixelFormat,
    pub dimension: ImageDimension,
}

impl Image {
    /// Create a 1x1 solid color pixel
    pub fn solid_color(color: [u8; 4]) -> Self {
        Self {
            data: color.to_vec(),
            width: 1,
            height: 1,
            format: PixelFormat::Rgba8Unorm,
            dimension: ImageDimension::D2,
        }
    }
}

#[derive(Clone, Copy)]
pub enum PixelFormat {
    Rgba8Unorm,
}

impl PixelFormat {
    pub fn to_wgpu(&self, color_space: ColorSpace) -> TextureFormat {
        match (self, color_space) {
            (PixelFormat::Rgba8Unorm, ColorSpace::Srgb) => TextureFormat::Rgba8UnormSrgb,
            (PixelFormat::Rgba8Unorm, ColorSpace::Linear) => TextureFormat::Rgba8Unorm,
        }
    }
}

#[derive(Clone, Copy)]
pub enum ImageDimension {
    D1,
    D2,
    D3,
}

impl From<ImageDimension> for TextureDimension {
    fn from(value: ImageDimension) -> Self {
        match value {
            ImageDimension::D1 => Self::D1,
            ImageDimension::D2 => Self::D2,
            ImageDimension::D3 => Self::D3,
        }
    }
}

impl From<ImageDimension> for TextureViewDimension {
    fn from(value: ImageDimension) -> Self {
        match value {
            ImageDimension::D1 => Self::D1,
            ImageDimension::D2 => Self::D2,
            ImageDimension::D3 => Self::D3,
        }
    }
}

#[derive(Clone, Copy)]
pub enum ColorSpace {
    Srgb,
    Linear,
}
