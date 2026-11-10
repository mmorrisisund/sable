use sable_core::handle::ImageHandle;
use wgpu::{AddressMode, FilterMode, MipmapFilterMode};

use crate::image::ColorSpace;

pub struct Texture {
    // pub name: String,
    pub sampler: TextureSampler,
    pub image: ImageHandle,
    pub color_space: ColorSpace,
}

pub struct TextureSampler {
    pub address_mode_u: AddressMode,
    pub address_mode_v: AddressMode,
    pub address_mode_w: AddressMode,
    pub mag_filter: FilterMode,
    pub min_filter: FilterMode,
    pub mipmap_filter: MipmapFilterMode,
}
