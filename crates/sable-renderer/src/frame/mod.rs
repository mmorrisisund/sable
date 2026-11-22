mod config;
mod context;
mod manager;
pub mod state;

use wgpu::{CommandEncoderDescriptor, TextureView};

pub use crate::frame::{config::FrameConfig, context::FrameContext, manager::FrameManager};
use crate::{RendererConfig, passes::RenderContext};

pub struct Frame<'a> {
    config: FrameConfig,
    ctx: FrameContext<'a>,
}

impl<'a> Frame<'a> {
    pub fn new(ctx: FrameContext<'a>, config: &RendererConfig) -> Self {
        let config = FrameConfig {
            width: config.width,
            height: config.height,
            depth_format: ctx.gpu_ctx.depth_format,
            surface_format: ctx.gpu_ctx.surface_format,
        };

        Self { config, ctx }
    }

    pub fn render(&mut self, view: &TextureView) {
        let ctx = RenderContext {
            gpu_ctx: self.ctx.gpu_ctx,
            commands: &self.ctx.commands,
            surface_view: view,
            depth_view: self.ctx.depth_view,
        };

        let mut encoder =
            self.ctx
                .gpu_ctx
                .device
                .create_command_encoder(&CommandEncoderDescriptor {
                    label: Some("main loop command encoder"),
                });

        self.ctx.basic_pass.render(ctx, &mut encoder);

        self.ctx.gpu_ctx.queue.submit(Some(encoder.finish()));
    }
}
