use wgpu::{
    Buffer, BufferUsages, Color, ColorTargetState, ColorWrites, Device, FragmentState, IndexFormat,
    LoadOp, MultisampleState, Operations, PipelineCompilationOptions, PipelineLayout,
    PipelineLayoutDescriptor, PrimitiveState, Queue, RenderPassColorAttachment,
    RenderPassDescriptor, RenderPipeline, RenderPipelineDescriptor, ShaderModule, StoreOp,
    TextureFormat, TextureView, VertexAttribute, VertexBufferLayout, VertexState, VertexStepMode,
    include_wgsl,
    util::{BufferInitDescriptor, DeviceExt},
};

use crate::{
    camera::Camera,
    mesh::{AttributeKind, Geometry, Mesh},
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
    pipeline: Option<RenderPipeline>,
    pipeline_layout: PipelineLayout,
    geometry: Option<GpuGeometry>,
    shader: ShaderModule,
}

impl Renderer {
    pub fn new(device: Device, queue: Queue, config: RendererConfig) -> Self {
        let shader = device.create_shader_module(include_wgsl!("shaders/basic.wgsl"));

        let pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[],
            immediate_size: 0,
        });

        Self {
            device,
            queue,
            config,
            pipeline: None,
            pipeline_layout,
            geometry: None,
            shader,
        }
    }

    pub fn add_mesh(&mut self, mesh: Mesh) {
        let geometry = GpuGeometry::new(&mesh.geometry, &self.device);

        let pipeline = self
            .device
            .create_render_pipeline(&RenderPipelineDescriptor {
                label: Some("renderer pipeline"),
                layout: Some(&self.pipeline_layout),
                vertex: VertexState {
                    module: &self.shader,
                    entry_point: Some("vs_main"),
                    compilation_options: Default::default(),
                    buffers: &geometry.layout_info.wgpu_layout(),
                },
                primitive: PrimitiveState::default(),
                depth_stencil: None,
                multisample: MultisampleState::default(),
                fragment: Some(FragmentState {
                    module: &self.shader,
                    entry_point: Some("fs_main"),
                    compilation_options: PipelineCompilationOptions::default(),
                    targets: &[Some(ColorTargetState {
                        format: self.config.color_format,
                        blend: None,
                        write_mask: ColorWrites::all(),
                    })],
                }),
                multiview_mask: None,
                cache: None,
            });

        self.geometry = Some(geometry);
        self.pipeline = Some(pipeline);
    }

    pub fn render(&self, _camera: &Camera, view: &TextureView) {
        let Some(geo) = self.geometry.as_ref() else {
            return;
        };

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
            rp.set_pipeline(self.pipeline.as_ref().unwrap());

            for (slot, buffer) in geo.buffers.iter().enumerate() {
                rp.set_vertex_buffer(slot as u32, buffer.slice(..));
            }

            if let Some(idx_buf) = geo.index_buffer.as_ref() {
                rp.set_index_buffer(idx_buf.slice(..), geo.index_format.unwrap());
                rp.draw_indexed(0..geo.count, 0, 0..1);
            } else {
                rp.draw(0..geo.count, 0..1);
            }
        }

        self.queue.submit(Some(encoder.finish()));
    }
}

struct GpuGeometry {
    layout_info: GeneratedVertexLayout,
    buffers: Vec<Buffer>,
    index_buffer: Option<Buffer>,
    index_format: Option<IndexFormat>,
    count: u32,
}

impl GpuGeometry {
    fn new(geometry: &Geometry, device: &Device) -> Self {
        let mut layouts = Vec::with_capacity(geometry.attributes().len());
        let mut buffers = Vec::new();

        for (kind, attr) in geometry.attributes() {
            layouts.push(OwnedVertexLayout {
                array_stride: attr.format.size(),
                step_mode: VertexStepMode::Vertex,
                attributes: vec![VertexAttribute {
                    format: attr.format,
                    offset: 0,
                    shader_location: kind.shader_location(),
                }],
            });

            buffers.push(device.create_buffer_init(&BufferInitDescriptor {
                label: None,
                contents: attr.data(),
                usage: BufferUsages::COPY_DST | BufferUsages::VERTEX,
            }));
        }

        let index_buffer = geometry.get_index_attribute().map(|attr| {
            device.create_buffer_init(&BufferInitDescriptor {
                label: Some("idx_buf"),
                contents: attr.data(),
                usage: BufferUsages::COPY_DST | BufferUsages::INDEX,
            })
        });

        let mut index_format = None;
        let count = if let Some(idx_attr) = geometry.get_index_attribute() {
            index_format = Some(idx_attr.format);
            idx_attr.count
        } else if let Some(attr) = geometry.get_attribute(AttributeKind::Position) {
            attr.count
        } else if let Some((_, attr)) = geometry.attributes().next() {
            attr.count
        } else {
            0
        };

        Self {
            layout_info: GeneratedVertexLayout { layouts },
            buffers,
            index_buffer,
            index_format,
            count,
        }
    }
}

struct GeneratedVertexLayout {
    layouts: Vec<OwnedVertexLayout>,
}

impl GeneratedVertexLayout {
    fn wgpu_layout(&self) -> Vec<Option<VertexBufferLayout<'_>>> {
        self.layouts.iter().map(|l| Some(l.as_wgpu())).collect()
    }
}

/// Mirror of ['VertexbufferLayout'] that owns the attributes
struct OwnedVertexLayout {
    array_stride: u64,
    step_mode: VertexStepMode,
    attributes: Vec<VertexAttribute>,
}

impl OwnedVertexLayout {
    fn as_wgpu(&self) -> VertexBufferLayout<'_> {
        VertexBufferLayout {
            array_stride: self.array_stride,
            step_mode: self.step_mode,
            attributes: &self.attributes,
        }
    }
}
