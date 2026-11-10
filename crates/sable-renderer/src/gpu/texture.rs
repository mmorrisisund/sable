use std::sync::Arc;

use sable_assets::AssetServer;
use sable_resources::texture::Texture;
use wgpu::{
    Extent3d, Sampler, SamplerDescriptor, TextureDescriptor, TextureUsages, TextureView,
    TextureViewDescriptor, util::DeviceExt, wgt::TextureDataOrder,
};

use crate::ResourceManager;

pub struct TextureBinding {
    #[allow(dead_code, reason = "texture kept alive for view")]
    texture: wgpu::Texture,
    pub texture_view: TextureView,
    pub sampler: Sampler,
}

//------------------------------
// ResourceManager extensions
//------------------------------
impl ResourceManager {
    pub fn get_or_create_texture(
        &mut self,
        texture: &Arc<Texture>,
        assets: &AssetServer,
    ) -> TextureBinding {
        let image = assets.image.get(texture.image).unwrap();
        let format = image.format.to_wgpu(texture.color_space);

        let gpu_texture = self.device.create_texture_with_data(
            &self.queue,
            &TextureDescriptor {
                label: None,
                size: Extent3d {
                    width: image.width,
                    height: image.height,
                    depth_or_array_layers: 1,
                },
                mip_level_count: 1,
                sample_count: 1,
                dimension: image.dimension.into(),
                format,
                usage: TextureUsages::RENDER_ATTACHMENT | TextureUsages::TEXTURE_BINDING,
                view_formats: &[],
            },
            TextureDataOrder::LayerMajor,
            &image.data,
        );

        let texture_view = gpu_texture.create_view(&TextureViewDescriptor {
            format: Some(format),
            dimension: Some(image.dimension.into()),
            ..Default::default()
        });

        let asset_sampler = &texture.sampler;
        let sampler = self.device.create_sampler(&SamplerDescriptor {
            address_mode_u: asset_sampler.address_mode_u,
            address_mode_v: asset_sampler.address_mode_v,
            address_mode_w: asset_sampler.address_mode_w,
            mag_filter: asset_sampler.mag_filter,
            min_filter: asset_sampler.min_filter,
            mipmap_filter: asset_sampler.mipmap_filter,
            ..Default::default()
        });

        TextureBinding {
            texture: gpu_texture,
            texture_view,
            sampler,
        }
    }
}
