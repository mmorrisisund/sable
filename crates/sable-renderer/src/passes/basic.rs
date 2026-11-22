use wgpu::{
    Color, CommandEncoder, LoadOp, Operations, RenderPassColorAttachment,
    RenderPassDepthStencilAttachment, RenderPassDescriptor, StoreOp,
};

use crate::passes::RenderContext;

pub struct BasicPass {}

impl BasicPass {
    pub fn render(&self, ctx: RenderContext<'_>, encoder: &mut CommandEncoder) {
        let mut rp = encoder.begin_render_pass(&RenderPassDescriptor {
            label: Some("render pass"),
            color_attachments: &[Some(RenderPassColorAttachment {
                view: ctx.surface_view,
                depth_slice: None,
                resolve_target: None,
                ops: Operations {
                    load: LoadOp::Clear(Color {
                        r: 0.1,
                        g: 0.2,
                        b: 0.3,
                        a: 1.0,
                    }),
                    store: StoreOp::Store,
                },
            })],
            depth_stencil_attachment: Some(RenderPassDepthStencilAttachment {
                view: ctx.depth_view,
                depth_ops: Some(Operations {
                    load: LoadOp::Load,
                    store: StoreOp::Store,
                }),
                stencil_ops: None,
            }),
            timestamp_writes: None,
            occlusion_query_set: None,
            multiview_mask: None,
        });

        rp.set_bind_group(0, &ctx.commands.frame_bg, &[]);

        for cmd in &ctx.commands.opaque {
            rp.set_pipeline(cmd.pipeline);

            rp.set_bind_group(1, cmd.material_bg, &[]);

            for (slot, buffer) in cmd.vertex_buffers().iter().enumerate() {
                rp.set_vertex_buffer(slot as u32, buffer.slice(..));
            }

            if let Some((idx_buf, count, format)) = cmd.index_buffer() {
                rp.set_index_buffer(idx_buf.slice(..), format);
                rp.draw_indexed(0..count, 0, 0..1);
            } else {
                rp.draw(cmd.instance_range(), 0..1);
            }
        }
    }
}
