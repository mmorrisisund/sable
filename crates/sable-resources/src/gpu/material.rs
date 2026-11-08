use bytemuck::{Pod, Zeroable, cast_slice};
use wgpu::{
    BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayout, BindGroupLayoutDescriptor,
    BindGroupLayoutEntry, BindingType, BufferBindingType, BufferUsages, Device, ShaderStages,
    util::{BufferInitDescriptor, DeviceExt},
};

use crate::Material;

pub struct GpuMaterial {
    bind_group: BindGroup,
    bind_group_layout: BindGroupLayout,
}

impl GpuMaterial {
    pub fn new(material: &Material, device: &Device) -> Self {
        let bind_group_layout = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: Some("material_bgl"),
            entries: &[BindGroupLayoutEntry {
                binding: 0,
                visibility: ShaderStages::VERTEX_FRAGMENT,
                ty: BindingType::Buffer {
                    ty: BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        });

        let buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("material_uniform"),
            contents: cast_slice(&[MaterialUniform::from(material)]),
            usage: BufferUsages::COPY_DST | BufferUsages::UNIFORM,
        });

        let bind_group = device.create_bind_group(&BindGroupDescriptor {
            label: Some("material_bg"),
            layout: &bind_group_layout,
            entries: &[BindGroupEntry {
                binding: 0,
                resource: buffer.as_entire_binding(),
            }],
        });

        Self {
            bind_group,
            bind_group_layout,
        }
    }

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
            base_color: mat.base_color.to_array(),
            metallic: mat.metallic,
            roughness: mat.roughness,
            _pad: [0.0; 2],
        }
    }
}

impl From<&Material> for MaterialUniform {
    fn from(mat: &Material) -> Self {
        Self {
            base_color: mat.base_color.to_array(),
            metallic: mat.metallic,
            roughness: mat.roughness,
            _pad: [0.0; 2],
        }
    }
}
