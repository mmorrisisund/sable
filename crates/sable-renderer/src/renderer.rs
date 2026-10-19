use wgpu::{
    Buffer, BufferDescriptor, BufferUsages, Color, ColorTargetState, ColorWrites, Device,
    FragmentState, LoadOp, MultisampleState, Operations, PipelineCompilationOptions,
    PipelineLayoutDescriptor, PrimitiveState, Queue, RenderPassColorAttachment,
    RenderPassDescriptor, RenderPipeline, RenderPipelineDescriptor, ShaderModuleDescriptor,
    ShaderSource, StoreOp, TextureFormat, TextureView, VertexBufferLayout, VertexState,
    vertex_attr_array,
};

use crate::camera::Camera;

pub struct RendererConfig {
    pub width: u32,
    pub height: u32,
    pub color_format: TextureFormat,
}

pub struct Renderer {
    device: Device,
    queue: Queue,
    config: RendererConfig,
    vertices: Buffer,
    pipeline: RenderPipeline,
}

impl Renderer {
    pub fn new(device: Device, queue: Queue, config: RendererConfig) -> Self {
        let shader = device.create_shader_module(ShaderModuleDescriptor {
            label: None,
            source: ShaderSource::Wgsl(SHADER.into()),
        });

        let layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[],
            immediate_size: 0,
        });

        let v_attr = vertex_attr_array![0 => Float32x3];
        let v_buf = VertexBufferLayout {
            array_stride: size_of::<[f32; 3]>() as u64,
            attributes: &v_attr,
            step_mode: wgpu::VertexStepMode::Vertex,
        };
        let pipeline = device.create_render_pipeline(&RenderPipelineDescriptor {
            label: Some("triangle pipeline"),
            layout: Some(&layout),
            vertex: VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                compilation_options: Default::default(),
                buffers: &[Some(v_buf)],
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

        let vertices = device.create_buffer(&BufferDescriptor {
            label: Some("Vertex buffer"),
            size: size_of::<[[f32; 3]; 3]>() as u64,
            usage: BufferUsages::COPY_DST | BufferUsages::VERTEX,
            mapped_at_creation: false,
        });

        Self {
            device,
            queue,
            config,
            vertices,
            pipeline,
        }
    }

    pub fn temp_add_tri(&mut self) {
        let vertices = vec![[-1.0f32, -1.0, 0.0], [0.0, 1.0, 0.0], [1.0, -1.0, 0.0]];
        let data = bytemuck::cast_slice(&vertices);

        self.queue.write_buffer(&self.vertices, 0, data);
    }

    pub fn render(&self, _camera: &Camera, view: &TextureView) {
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
            rp.set_vertex_buffer(0, self.vertices.slice(..));
            rp.draw(0..3, 0..1);
        }

        self.queue.submit(Some(encoder.finish()));
    }
}

const SHADER: &str = r#"
    @vertex
    fn vs_main(@location(0) pos: vec3f) -> @builtin(position) vec4f {
        return vec4f(pos, 1.0);
    }

    @fragment
    fn fs_main() -> @location(0) vec4f {
        return vec4f(1.0, 0.0, 0.0, 1.0);
    }
"#;
