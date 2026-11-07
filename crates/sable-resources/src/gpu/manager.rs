use sable_core::handle::GeometryHandle;
use slotmap::{SecondaryMap, SlotMap};
use wgpu::RenderPipeline;

use crate::GpuGeometry;

#[derive(Default)]
pub struct ResourceManager {
    pub geometry: SlotMap<GeometryHandle, GpuGeometry>,
    pub pipelines: SecondaryMap<GeometryHandle, RenderPipeline>,
}
