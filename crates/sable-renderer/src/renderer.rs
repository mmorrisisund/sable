use bytemuck::cast_slice;
use gpui::WgpuSurfaceHandle;
use sable_assets::AssetServer;
use sable_resources::{CameraUniform, RenderCamera, Scene};
use wgpu::{
    BindGroup, BindGroupDescriptor, BindGroupEntry, Buffer, BufferDescriptor, BufferUsages, Color,
    Extent3d, LoadOp, Operations, RenderPassColorAttachment, RenderPassDepthStencilAttachment,
    RenderPassDescriptor, StoreOp, Texture, TextureDescriptor, TextureFormat, TextureUsages,
    TextureView, TextureViewDescriptor,
};

use crate::{
    ResourceManager,
    core::scene::RenderableItem,
    frame::{Frame, FrameManager},
    gpu::context::WgpuContext,
};

pub struct RendererConfig {
    pub width: u32,
    pub height: u32,
    pub color_format: TextureFormat,
}

pub struct Renderer {
    gpu_ctx: WgpuContext,
    resources: ResourceManager,
    frame_manager: FrameManager,

    camera_bg: Option<BindGroup>,
    camera_uniform: Option<Buffer>,

    renderables: Vec<RenderableItem>,

    depth_texture: Texture,
    depth_view: TextureView,
}

impl Renderer {
    pub fn new(surface: WgpuSurfaceHandle) -> Self {
        let device = surface.device();
        let queue = surface.queue();
        let (w, h) = surface.size();

        let depth_texture = device.create_texture(&TextureDescriptor {
            label: Some("depth_t"),
            size: Extent3d {
                width: w,
                height: h,
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
        let resources = ResourceManager::new(device.clone(), queue.clone());

        let gpu_ctx = WgpuContext::from_surface_handle(&surface);
        let frame_manager = FrameManager::new();

        Self {
            gpu_ctx,
            resources,
            frame_manager,
            camera_bg: None,
            camera_uniform: None,
            renderables: Vec::new(),
            depth_texture,
            depth_view,
        }
    }

    pub fn begin_frame(&mut self, scene: &Scene, assets: &AssetServer) -> Option<Frame<'_>> {
        // extract renderables from scene
        self.renderables = self
            .frame_manager
            .extract_scene(scene, assets, &mut self.resources);

        // create commands
        // need renderable scene, frame state, gpu ctx, resources, pipelines, shaders

        // prepare pass (maybe)

        // return frame

        None
    }

    pub fn end_frame(&mut self) {
        // future use
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        let depth_texture = self.gpu_ctx.device.create_texture(&TextureDescriptor {
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
        self.gpu_ctx
            .queue
            .write_buffer(cam_uni, 0, cast_slice(&[camera.extract_uniform()]));

        let Some(cam_bg) = self.camera_bg.as_ref() else {
            return;
        };
        let mut encoder = self
            .gpu_ctx
            .device
            .create_command_encoder(&Default::default());

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

            for renderable in &self.renderables {
                let geometry = self
                    .resources
                    .geometry
                    .get(renderable.geometry)
                    .expect("resources must exist");
                let material = self
                    .resources
                    .material
                    .get(renderable.material)
                    .expect("resources must exist");
                let pipeline = self
                    .resources
                    .pipelines
                    .get(renderable.geometry)
                    .expect("pipeline must exist");

                rp.set_pipeline(pipeline);
                rp.set_bind_group(0, cam_bg, &[]);

                rp.set_bind_group(1, material.bind_group(), &[]);

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

        self.gpu_ctx.queue.submit(Some(encoder.finish()));
    }
}

impl Renderer {
    pub fn prepare_camera(&mut self) {
        let uniform = self.gpu_ctx.device.create_buffer(&BufferDescriptor {
            label: Some("camera uniform"),
            size: size_of::<CameraUniform>() as u64,
            usage: BufferUsages::COPY_DST | BufferUsages::UNIFORM,
            mapped_at_creation: false,
        });

        self.camera_bg = Some(self.gpu_ctx.device.create_bind_group(&BindGroupDescriptor {
            label: Some("camera bg"),
            layout: &self.resources.camera_layout,
            entries: &[BindGroupEntry {
                binding: 0,
                resource: uniform.as_entire_binding(),
            }],
        }));

        self.camera_uniform = Some(uniform);
    }
}
