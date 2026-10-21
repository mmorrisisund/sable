use wgpu::{
    Buffer, BufferUsages, Color, ColorTargetState, ColorWrites, Device, FragmentState, LoadOp,
    MultisampleState, Operations, PipelineCompilationOptions, PipelineLayoutDescriptor,
    PrimitiveState, Queue, RenderPassColorAttachment, RenderPassDescriptor, RenderPipeline,
    RenderPipelineDescriptor, StoreOp, TextureFormat, TextureView, VertexState, include_wgsl,
    util::{BufferInitDescriptor, DeviceExt},
};

use crate::{
    camera::Camera,
    mesh::{Mesh, Vertex},
};

pub struct RendererConfig {
    pub width: u32,
    pub height: u32,
    pub color_format: TextureFormat,
}

pub struct Renderer {
    device: Device,
    queue: Queue,
    config: RendererConfig,
    pipeline: RenderPipeline,
    mesh: Option<Mesh>,
    v_buf: Option<Buffer>,
}

impl Renderer {
    pub fn new(device: Device, queue: Queue, config: RendererConfig) -> Self {
        let shader = device.create_shader_module(include_wgsl!("shaders/basic.wgsl"));

        let layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[],
            immediate_size: 0,
        });

        let pipeline = device.create_render_pipeline(&RenderPipelineDescriptor {
            label: Some("triangle pipeline"),
            layout: Some(&layout),
            vertex: VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                compilation_options: Default::default(),
                buffers: &[Some(Vertex::layout())],
            },
            primitive: PrimitiveState::default(),
            depth_stencil: None,
            multisample: MultisampleState::default(),
            fragment: Some(FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                compilation_options: PipelineCompilationOptions::default(),
                targets: &[Some(ColorTargetState {
                    format: config.color_format,
                    blend: None,
                    write_mask: ColorWrites::all(),
                })],
            }),
            multiview_mask: None,
            cache: None,
        });

        Self {
            device,
            queue,
            config,
            pipeline,
            mesh: None,
            v_buf: None,
        }
    }

    pub fn add_mesh(&mut self, mesh: Mesh) {
        self.v_buf = Some(self.device.create_buffer_init(&BufferInitDescriptor {
            label: Some("v_buf"),
            contents: bytemuck::cast_slice(&mesh.vertices),
            usage: BufferUsages::COPY_DST | BufferUsages::VERTEX,
        }));
        self.mesh = Some(mesh);
    }

    pub fn render(&self, _camera: &Camera, view: &TextureView) {
        let v_buf = self.v_buf.as_ref().unwrap();
        let mut encoder = self.device.create_command_encoder(&Default::default());

        {
            let mut rp = encoder.begin_render_pass(&RenderPassDescriptor {
                label: Some("render pass"),
                color_attachments: &[Some(RenderPassColorAttachment {
                    view,
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
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
                multiview_mask: None,
            });

            rp.set_pipeline(&self.pipeline);
            rp.set_vertex_buffer(0, v_buf.slice(..));
            rp.draw(0..3, 0..1);
        }

        self.queue.submit(Some(encoder.finish()));
    }
}
