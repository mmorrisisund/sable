use sable_core::handle::{GeometryHandle, MaterialHandle};
use slotmap::{SecondaryMap, SlotMap};
use wgpu::RenderPipeline;

use crate::{GpuGeometry, GpuMaterial};

#[derive(Default)]
pub struct ResourceManager {
    pub geometry: SlotMap<GeometryHandle, GpuGeometry>,
    pub material: SlotMap<MaterialHandle, GpuMaterial>,
    pub pipelines: SecondaryMap<GeometryHandle, RenderPipeline>,
}
