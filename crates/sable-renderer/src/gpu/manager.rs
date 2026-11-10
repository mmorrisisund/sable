use sable_core::handle::{GeometryHandle, MaterialHandle};
use slotmap::{SecondaryMap, SlotMap};
use wgpu::{Device, Queue, RenderPipeline};

use crate::{GpuGeometry, GpuMaterial};

pub struct ResourceManager {
    pub(super) device: Device,
    pub(super) queue: Queue,

    pub geometry: SlotMap<GeometryHandle, GpuGeometry>,
    pub material: SlotMap<MaterialHandle, GpuMaterial>,
    pub pipelines: SecondaryMap<GeometryHandle, RenderPipeline>,
}

impl ResourceManager {
    pub fn new(device: Device, queue: Queue) -> Self {
        Self {
            device,
            queue,
            geometry: SlotMap::default(),
            material: SlotMap::default(),
            pipelines: SecondaryMap::default(),
        }
    }
}
