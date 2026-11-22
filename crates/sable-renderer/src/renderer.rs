use gpui::WgpuSurfaceHandle;
use sable_assets::AssetServer;
use sable_resources::{RenderCamera, Scene};
use wgpu::{
    BindGroup, BindGroupDescriptor, BindGroupEntry, Buffer, BufferDescriptor, BufferUsages,
    Extent3d, Texture, TextureDescriptor, TextureFormat, TextureUsages, TextureView,
    TextureViewDescriptor,
};

use crate::{
    ResourceManager,
    core::{commands::render::RenderCommands, scene::RenderableItem},
    frame::{Frame, FrameContext, FrameManager},
    gpu::context::WgpuContext,
    passes::basic::BasicPass,
};

pub struct RendererConfig {
    pub width: u32,
    pub height: u32,
    pub color_format: TextureFormat,
}

pub struct Renderer {
    config: RendererConfig,
    gpu_ctx: WgpuContext,
    resources: ResourceManager,
    frame_manager: FrameManager,

    depth_texture: Texture,
    depth_view: TextureView,

    // Various passes, put these into a single struct?
    basic_pass: BasicPass,
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

        let config = RendererConfig {
            width: w,
            height: h,
            color_format: surface.format(),
        };
        let gpu_ctx = WgpuContext::from_surface_handle(&surface);
        let frame_manager = FrameManager::new();

        Self {
            config,
            gpu_ctx,
            resources,
            frame_manager,
            depth_texture,
            depth_view,

            basic_pass: BasicPass {},
        }
    }

    pub fn begin_frame(&mut self, scene: &Scene, assets: &AssetServer) -> Frame<'_> {
        // Clear transient data
        self.reset();

        // extract renderables from scene
        self.frame_manager
            .extract_scene(scene, assets, &mut self.resources);

        // create commands - package of resources used in pass. 1 cmd per renderable item
        // need renderable scene, frame state, gpu ctx, resources, pipelines, shaders
        // Command { mesh bg, geo_handle, mat_handle, pipeline_id }
        // just ids to resources (except mesh binding)
        // full list contains global bg
        let commands =
            RenderCommands::generate(&self.frame_manager.scene.items, &mut self.resources);
        // prepare pass (maybe). setup renderpasses

        let ctx = FrameContext {
            gpu_ctx: &self.gpu_ctx,
            basic_pass: &mut self.basic_pass,
            commands,
            depth_view: &self.depth_view,
        };

        Frame::new(ctx, &self.config)
    }

    pub fn end_frame(&mut self) {
        // future use
    }

    /// Central point for all reset activities.  Called at start of begin_frame
    fn reset(&mut self) {
        self.frame_manager.scene.items.clear();
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
}
