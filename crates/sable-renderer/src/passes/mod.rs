use wgpu::TextureView;

use crate::{core::commands::render::RenderCommands, gpu::context::WgpuContext};

pub mod basic;

pub struct RenderContext<'a> {
    pub gpu_ctx: &'a WgpuContext,
    pub commands: &'a RenderCommands<'a>,

    pub surface_view: &'a TextureView,
    pub depth_view: &'a TextureView,
}
