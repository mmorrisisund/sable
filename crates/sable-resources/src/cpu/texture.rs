use std::borrow::Cow;

use sable_core::handle::ImageHandle;
use wgpu::{AddressMode, FilterMode, MipmapFilterMode};

use crate::image::ColorSpace;

pub struct Texture {
    pub name: Option<Cow<'static, str>>,
    pub sampler: TextureSampler,
    pub image: ImageHandle,
    pub color_space: ColorSpace,
}

impl Texture {
    pub fn new_2d(name: Option<&str>, image: ImageHandle) -> Self {
        Self {
            name: name.map(|s| Cow::Owned(s.to_string())),
            sampler: TextureSampler::default(),
            image,
            color_space: ColorSpace::Linear,
        }
    }
}

#[derive(Default)]
pub struct TextureSampler {
    pub address_mode_u: AddressMode,
    pub address_mode_v: AddressMode,
    pub address_mode_w: AddressMode,
    pub mag_filter: FilterMode,
    pub min_filter: FilterMode,
    pub mipmap_filter: MipmapFilterMode,
}
