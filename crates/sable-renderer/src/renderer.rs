use bytemuck::{Pod, Zeroable, cast_slice};
use wgpu::{
    BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayout, BindGroupLayoutDescriptor,
    BindGroupLayoutEntry, BindingType, Buffer, BufferBindingType, BufferUsages, Color,
    ColorTargetState, ColorWrites, Device, FragmentState, IndexFormat, LoadOp, MultisampleState,
    Operations, PipelineCompilationOptions, PipelineLayoutDescriptor, PrimitiveState, Queue,
    RenderPassColorAttachment, RenderPassDescriptor, RenderPipeline, RenderPipelineDescriptor,
    ShaderModule, ShaderStages, StoreOp, TextureFormat, TextureView, VertexAttribute,
    VertexBufferLayout, VertexState, VertexStepMode, include_wgsl,
    util::{BufferInitDescriptor, DeviceExt},
};

use sable_resources::{
    geometry::{AttributeKind, Geometry},
    material::Material,
    mesh::Mesh,
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
    pipeline: Option<RenderPipeline>,
    geometry: Option<GpuGeometry>,
    material: Option<GpuMaterial>,
    shader: ShaderModule,
}

impl Renderer {
    pub fn new(device: Device, queue: Queue, config: RendererConfig) -> Self {
        let shader = device.create_shader_module(include_wgsl!("shaders/basic.wgsl"));

        Self {
            device,
            queue,
            config,
            pipeline: None,
            geometry: None,
            material: None,
            shader,
        }
    }

    pub fn add_mesh(&mut self, mesh: Mesh) {
        let geometry = GpuGeometry::new(&mesh.geometry, &self.device);
        let material = GpuMaterial::new(&mesh.material, &self.device);

        let pipeline_layout = self
            .device
            .create_pipeline_layout(&PipelineLayoutDescriptor {
                label: None,
                bind_group_layouts: &[Some(&material.bind_group_layout)],
                immediate_size: 0,
            });

        let pipeline = self
            .device
            .create_render_pipeline(&RenderPipelineDescriptor {
                label: Some("renderer pipeline"),
                layout: Some(&pipeline_layout),
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
        self.material = Some(material);
        self.pipeline = Some(pipeline);
    }

    pub fn render(&self, _camera: &Camera, view: &TextureView) {
        let Some(geo) = self.geometry.as_ref() else {
            return;
        };
        let Some(mat) = self.material.as_ref() else {
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
            rp.set_bind_group(0, &mat.bind_group, &[]);

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

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
struct MaterialUniform {
    base_color: [f32; 4],
    metallic: f32,
    roughness: f32,
    _pad: [f32; 2],
}

impl From<Material> for MaterialUniform {
    fn from(mat: Material) -> Self {
        Self {
            base_color: mat.base_color.to_array(),
            metallic: mat.metallic,
            roughness: mat.roughness,
            _pad: [0.0; 2],
        }
    }
}

impl From<&Material> for MaterialUniform {
    fn from(mat: &Material) -> Self {
        Self {
            base_color: mat.base_color.to_array(),
            metallic: mat.metallic,
            roughness: mat.roughness,
            _pad: [0.0; 2],
        }
    }
}

struct GpuMaterial {
    bind_group: BindGroup,
    bind_group_layout: BindGroupLayout,
}

impl GpuMaterial {
    fn new(material: &Material, device: &Device) -> Self {
        let bind_group_layout = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: Some("material_bgl"),
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

        let buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("material_uniform"),
            contents: cast_slice(&[MaterialUniform::from(material)]),
            usage: BufferUsages::COPY_DST | BufferUsages::UNIFORM,
        });

        let bind_group = device.create_bind_group(&BindGroupDescriptor {
            label: Some("material_bg"),
            layout: &bind_group_layout,
            entries: &[BindGroupEntry {
                binding: 0,
                resource: buffer.as_entire_binding(),
            }],
        });

        Self {
            bind_group,
            bind_group_layout,
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
