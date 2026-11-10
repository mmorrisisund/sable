use bytemuck::{Pod, Zeroable, cast_slice};
use sable_assets::AssetServer;
use sable_core::handle::MaterialHandle;
use sable_resources::Material;
use wgpu::{
    BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayout, BindGroupLayoutDescriptor,
    BindGroupLayoutEntry, BindingResource, BindingType, BufferBindingType, BufferUsages,
    SamplerBindingType, ShaderStages, TextureSampleType, TextureViewDimension,
    util::{BufferInitDescriptor, DeviceExt},
};

use crate::ResourceManager;

pub struct GpuMaterial {
    bind_group: BindGroup,
    bind_group_layout: BindGroupLayout,
}

impl ResourceManager {
    pub fn add_material(&mut self, handle: MaterialHandle, assets: &AssetServer) -> MaterialHandle {
        if self.material.get(handle).is_some() {
            return handle;
        }

        let material_asset = assets
            .material
            .get(handle)
            .expect("GPU material resources must be created");
        let texture_asset = assets
            .texture
            .get(material_asset.base_color_texture)
            .unwrap();

        let texture = self.get_or_create_texture(&texture_asset, assets);

        let bind_group_layout = self
            .device
            .create_bind_group_layout(&BindGroupLayoutDescriptor {
                label: Some("material_bgl"),
                entries: &[
                    BindGroupLayoutEntry {
                        binding: 0,
                        visibility: ShaderStages::VERTEX_FRAGMENT,
                        ty: BindingType::Buffer {
                            ty: BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    },
                    BindGroupLayoutEntry {
                        binding: 1,
                        visibility: ShaderStages::FRAGMENT,
                        ty: BindingType::Texture {
                            sample_type: TextureSampleType::Float { filterable: true },
                            view_dimension: TextureViewDimension::D2,
                            multisampled: false,
                        },
                        count: None,
                    },
                    BindGroupLayoutEntry {
                        binding: 2,
                        visibility: ShaderStages::FRAGMENT,
                        ty: BindingType::Sampler(SamplerBindingType::Filtering),
                        count: None,
                    },
                ],
            });

        let buffer = self.device.create_buffer_init(&BufferInitDescriptor {
            label: Some("material_uniform"),
            contents: cast_slice(&[MaterialUniform::from(material_asset.as_ref())]),
            usage: BufferUsages::COPY_DST | BufferUsages::UNIFORM,
        });

        let bind_group = self.device.create_bind_group(&BindGroupDescriptor {
            label: Some("material_bg"),
            layout: &bind_group_layout,
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: buffer.as_entire_binding(),
                },
                BindGroupEntry {
                    binding: 1,
                    resource: BindingResource::TextureView(&texture.texture_view),
                },
                BindGroupEntry {
                    binding: 2,
                    resource: BindingResource::Sampler(&texture.sampler),
                },
            ],
        });

        self.material.insert(GpuMaterial {
            bind_group,
            bind_group_layout,
        })
    }
}

impl GpuMaterial {
    pub fn bind_group(&self) -> &BindGroup {
        &self.bind_group
    }

    pub fn layout(&self) -> &BindGroupLayout {
        &self.bind_group_layout
    }
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
struct MaterialUniform {
    base_color: [f32; 4],
    metallic: f32,
    roughness: f32,
    _pad: [f32; 2],
}

impl From<Material> for MaterialUniform {
    fn from(mat: Material) -> Self {
        Self {
            base_color: mat.base_color_factor.to_array(),
            metallic: mat.metallic_factor,
            roughness: mat.roughness_factor,
            _pad: [0.0; 2],
        }
    }
}

impl From<&Material> for MaterialUniform {
    fn from(mat: &Material) -> Self {
        Self {
            base_color: mat.base_color_factor.to_array(),
            metallic: mat.metallic_factor,
            roughness: mat.roughness_factor,
            _pad: [0.0; 2],
        }
    }
}

//------------------------------
// ResourceManager extensions
//------------------------------
