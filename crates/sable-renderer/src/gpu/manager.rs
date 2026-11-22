use sable_core::handle::{GeometryHandle, MaterialHandle};
use slotmap::{SecondaryMap, SlotMap};
use wgpu::{
    BindGroupLayout, BindGroupLayoutDescriptor, BindGroupLayoutEntry, BindingType,
    BufferBindingType, Device, Queue, RenderPipeline, ShaderModule, ShaderStages, include_wgsl,
};

use crate::{GpuGeometry, GpuMaterial, gpu::binding::FrameBindingContext};

pub struct ResourceManager {
    pub(crate) device: Device,
    pub(crate) queue: Queue,
    // pub(super) sys_textures: SystemTextures,
    pub geometry: SlotMap<GeometryHandle, GpuGeometry>,
    pub material: SlotMap<MaterialHandle, GpuMaterial>,
    pub pipelines: SecondaryMap<GeometryHandle, RenderPipeline>,

    // bindings
    pub frame_globals: Option<FrameBindingContext>,

    // temp stuff - resources
    pub shader: ShaderModule,
}

impl ResourceManager {
    pub fn new(device: Device, queue: Queue) -> Self {
        // let sys_textures = SystemTextures::new(&device, &queue);
        let shader = device.create_shader_module(include_wgsl!("../passes/shaders/basic.wgsl"));
        let camera_layout = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: Some("camera bgl"),
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

        Self {
            device,
            queue,
            // sys_textures,
            shader,
            geometry: SlotMap::default(),
            material: SlotMap::default(),
            pipelines: SecondaryMap::default(),

            frame_globals: None,
        }
    }
}
