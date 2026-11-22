use sable_core::handle::{GeometryHandle, MaterialHandle};
use slotmap::{SecondaryMap, SlotMap};
use wgpu::{Device, Queue, RenderPipeline, ShaderModule, include_wgsl};

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
