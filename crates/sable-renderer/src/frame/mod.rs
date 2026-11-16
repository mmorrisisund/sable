mod config;
mod context;
mod manager;

use wgpu::TextureView;

pub use crate::frame::{config::FrameConfig, context::FrameContext, manager::FrameManager};

pub struct Frame<'a> {
    config: FrameConfig,
    ctx: FrameContext<'a>,
}

impl<'a> Frame<'a> {
    pub fn new(ctx: FrameContext<'a>, (width, height): (u32, u32)) -> Self {
        let config = FrameConfig {
            width,
            height,
            depth_format: ctx.gpu_ctx.depth_format,
            surface_format: ctx.gpu_ctx.surface_format,
        };

        Self { config, ctx }
    }

    pub fn render(&mut self, _view: &TextureView) {}
}
