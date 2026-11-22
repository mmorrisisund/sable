//! ResourceManager extensions for bind group management

use wgpu::{
    BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayout, BindGroupLayoutEntry,
    BindingType, Buffer, BufferBindingType, BufferDescriptor, BufferUsages, ShaderStages,
};

use crate::{frame::state::FrameState, gpu::manager::ResourceManager};

pub struct FrameBindingContext {
    buffer: Buffer,
    pub bindgroup: BindGroup,
    pub layout: BindGroupLayout,
}

impl ResourceManager {
    /// Update buffer for frame globals.  Created if needed
    pub fn prepare_frame_binding(&mut self, state: &FrameState) {
        let buffer = if let Some(ctx) = self.frame_globals.as_ref() {
            &ctx.buffer
        } else {
            let buffer = self.device.create_buffer(&BufferDescriptor {
                label: Some("Frame uniform"),
                size: size_of::<FrameState>() as u64,
                mapped_at_creation: false,
                usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
            });

            let layout = self
                .device
                .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                    label: Some("Frame binding layout (group 0)"),
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

            let bindgroup = self.device.create_bind_group(&BindGroupDescriptor {
                label: Some("Frame binding (group 0)"),
                layout: &layout,
                entries: &[BindGroupEntry {
                    binding: 0,
                    resource: buffer.as_entire_binding(),
                }],
            });

            self.frame_globals = Some(FrameBindingContext {
                buffer,
                bindgroup,
                layout,
            });

            &self.frame_globals.as_ref().unwrap().buffer
        };

        self.queue
            .write_buffer(buffer, 0, bytemuck::cast_slice(&[*state]));
    }
}
