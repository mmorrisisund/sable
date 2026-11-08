use bytemuck::{Pod, Zeroable, cast_slice};
use sable_assets::AssetServer;
use sable_resources::{GpuGeometry, ResourceManager, Scene, material::Material};
use wgpu::{
    BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayout, BindGroupLayoutDescriptor,
    BindGroupLayoutEntry, BindingType, Buffer, BufferBindingType, BufferDescriptor, BufferUsages,
    Color, ColorTargetState, ColorWrites, DepthStencilState, Device, Extent3d, FragmentState,
    LoadOp, MultisampleState, Operations, PipelineCompilationOptions, PipelineLayoutDescriptor,
    PrimitiveState, Queue, RenderPassColorAttachment, RenderPassDepthStencilAttachment,
    RenderPassDescriptor, RenderPipelineDescriptor, ShaderModule, ShaderStages, StoreOp, Texture,
    TextureDescriptor, TextureFormat, TextureUsages, TextureView, TextureViewDescriptor,
    VertexState, include_wgsl,
    util::{BufferInitDescriptor, DeviceExt},
};

use crate::{RenderCamera, camera::CameraUniform};

pub struct RendererConfig {
    pub width: u32,
    pub height: u32,
    pub color_format: TextureFormat,
}

pub struct Renderer {
    device: Device,
    queue: Queue,
    config: RendererConfig,

    assets: AssetServer,
    resources: ResourceManager,

    scene: Option<Scene>,

    shader: ShaderModule,
    camera_layout: BindGroupLayout,
    camera_bg: Option<BindGroup>,
    camera_uniform: Option<Buffer>,

    depth_texture: Texture,
    depth_view: TextureView,
}

impl Renderer {
    pub fn new(device: Device, queue: Queue, config: RendererConfig, assets: AssetServer) -> Self {
        let shader = device.create_shader_module(include_wgsl!("shaders/basic.wgsl"));
        let camera_layout = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: Some("camera bgl"),
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

        let depth_texture = device.create_texture(&TextureDescriptor {
            label: Some("depth_t"),
            size: Extent3d {
                width: config.width,
                height: config.height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: TextureFormat::Depth32Float,
            usage: TextureUsages::COPY_DST | TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        });
        let depth_view = depth_texture.create_view(&TextureViewDescriptor::default());

        Self {
            device,
            queue,
            config,
            assets,
            resources: ResourceManager::default(),
            shader,
            camera_layout,
            camera_bg: None,
            camera_uniform: None,
            scene: None,

            depth_texture,
            depth_view,
        }
    }

    pub fn set_scene(&mut self, scene: Scene) {
        self.scene = Some(scene);
        self.extract_scene();
    }

    pub fn start_frame(&mut self) {
        let Some(scene) = &mut self.scene else {
            return;
        };

        scene.update();
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        let depth_texture = self.device.create_texture(&TextureDescriptor {
            label: Some("depth_t"),
            size: Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: TextureFormat::Depth32Float,
            usage: TextureUsages::COPY_DST | TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        });
        let depth_view = depth_texture.create_view(&TextureViewDescriptor::default());

        self.depth_texture = depth_texture;
        self.depth_view = depth_view;
    }

    pub fn render(&self, camera: &impl RenderCamera, view: &TextureView) {
        let Some(cam_uni) = self.camera_uniform.as_ref() else {
            return;
        };
        self.queue
            .write_buffer(cam_uni, 0, cast_slice(&[camera.extract_uniform()]));

        let Some(cam_bg) = self.camera_bg.as_ref() else {
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
                depth_stencil_attachment: Some(RenderPassDepthStencilAttachment {
                    view: &self.depth_view,
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

            for (handle, geometry) in &self.resources.geometry {
                let pipeline = self
                    .resources
                    .pipelines
                    .get(handle)
                    .expect("pipeline must exist");
                rp.set_pipeline(pipeline);
                rp.set_bind_group(0, cam_bg, &[]);
                // rp.set_bind_group(0, &mat.bind_group, &[]);

                for (slot, buffer) in geometry.vertex_buffers().iter().enumerate() {
                    rp.set_vertex_buffer(slot as u32, buffer.slice(..));
                }

                if let Some(idx_buf) = geometry.index_buffer() {
                    rp.set_index_buffer(idx_buf.slice(..), *geometry.index_format().unwrap());
                    rp.draw_indexed(0..geometry.count(), 0, 0..1);
                } else {
                    rp.draw(0..geometry.count(), 0..1);
                }
            }
        }

        self.queue.submit(Some(encoder.finish()));
    }
}

impl Renderer {
    fn extract_scene(&mut self) {
        let Some(scene) = self.scene.as_ref() else {
            return;
        };

        for node in scene.nodes() {
            if let Some(mesh) = scene.get_mesh(node)
                && let Some(mesh_geo) = self.assets.geometry.get(mesh.geometry)
            {
                let geo = GpuGeometry::new(&mesh_geo, &self.device);
                self.resources.geometry.insert(geo);
            }
        }

        let pipeline_layout = self
            .device
            .create_pipeline_layout(&PipelineLayoutDescriptor {
                label: None,
                // bind_group_layouts: &[Some(&material.bind_group_layout)],
                // bind_group_layouts: &[],
                bind_group_layouts: &[Some(&self.camera_layout)],
                immediate_size: 0,
            });

        for (geo_handle, gpu_geo) in &self.resources.geometry {
            let pipeline = self
                .device
                .create_render_pipeline(&RenderPipelineDescriptor {
                    label: Some("renderer pipeline"),
                    layout: Some(&pipeline_layout),
                    vertex: VertexState {
                        module: &self.shader,
                        entry_point: Some("vs_main"),
                        compilation_options: Default::default(),
                        buffers: &gpu_geo.vertex_layout().wgpu_layout(),
                    },
                    primitive: PrimitiveState {
                        topology: wgpu::PrimitiveTopology::TriangleList, // TODO: Get from model
                        ..Default::default()
                    },
                    depth_stencil: Some(DepthStencilState {
                        format: TextureFormat::Depth32Float,
                        depth_compare: Some(wgpu::CompareFunction::Always),
                        depth_write_enabled: Some(true),
                        stencil: Default::default(),
                        bias: Default::default(),
                    }),
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

            // self.material = Some(material);
            self.resources.pipelines.insert(geo_handle, pipeline);
        }

        let uniform = self.device.create_buffer(&BufferDescriptor {
            label: Some("camera uniform"),
            size: size_of::<CameraUniform>() as u64,
            usage: BufferUsages::COPY_DST | BufferUsages::UNIFORM,
            mapped_at_creation: false,
        });

        self.camera_bg = Some(self.device.create_bind_group(&BindGroupDescriptor {
            label: Some("camera bg"),
            layout: &self.camera_layout,
            entries: &[BindGroupEntry {
                binding: 0,
                resource: uniform.as_entire_binding(),
            }],
        }));

        self.camera_uniform = Some(uniform);
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
