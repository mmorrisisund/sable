use std::sync::Arc;

use sable_assets::AssetServer;
use sable_resources::texture::Texture;
use wgpu::{
    Extent3d, Sampler, SamplerDescriptor, TextureDescriptor, TextureUsages, TextureView,
    TextureViewDescriptor, util::DeviceExt, wgt::TextureDataOrder,
};

use crate::ResourceManager;

pub struct TextureBinding {
    #[allow(dead_code, reason = "texture kept alive for view")] // TODO: Is this actually needed?
    texture: wgpu::Texture,
    pub texture_view: TextureView,
    pub sampler: Sampler,
}

//------------------------------
// ResourceManager extensions
//------------------------------
impl ResourceManager {
    pub fn get_or_create_texture_binding(
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

// /// Global fallback textures
// pub struct SystemTextures {
//     pub white_2d: TextureView,
//     pub black_2d: TextureView,
//     pub normal_2d: TextureView,
// }

// impl SystemTextures {
//     pub fn new(device: &Device, queue: &Queue) -> Self {
//         let white_2d = create_1x1_rgba(device, queue, [255, 255, 255, 255], "sys_white_2d");
//         let black_2d = create_1x1_rgba(device, queue, [0, 0, 0, 255], "sys_black_2d");
//         let normal_2d = create_1x1_rgba(device, queue, [128, 128, 255, 255], "sys_normal_2d");

//         Self {
//             white_2d,
//             black_2d,
//             normal_2d,
//         }
//     }
// }

// fn create_1x1_rgba(
//     device: &Device,
//     queue: &Queue,
//     color: [u8; 4],
//     name: &'static str,
// ) -> TextureView {
//     let texture = device.create_texture_with_data(
//         queue,
//         &TextureDescriptor {
//             label: Some(name),
//             size: Extent3d {
//                 width: 1,
//                 height: 1,
//                 depth_or_array_layers: 1,
//             },
//             mip_level_count: 1,
//             sample_count: 1,
//             dimension: TextureDimension::D2,
//             format: TextureFormat::Rgba8Unorm,
//             usage: TextureUsages::TEXTURE_BINDING,
//             view_formats: &[],
//         },
//         TextureDataOrder::LayerMajor,
//         &color,
//     );

//     texture.create_view(&TextureViewDescriptor::default())
// }
