use wgpu::TextureView;

use crate::{
    core::commands::render::RenderCommands, gpu::context::WgpuContext, passes::basic::BasicPass,
};

/// holds resources to draw frame.  references to resources, passes, etc.
pub struct FrameContext<'a> {
    pub gpu_ctx: &'a WgpuContext,
    pub commands: RenderCommands<'a>,
    pub depth_view: &'a TextureView,
    pub basic_pass: &'a mut BasicPass,
}
