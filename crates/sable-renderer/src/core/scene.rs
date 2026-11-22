use sable_assets::AssetServer;
use sable_core::handle::{GeometryHandle, MaterialHandle};
use sable_resources::Scene;
use wgpu::{
    ColorTargetState, ColorWrites, DepthStencilState, Face, FragmentState, MultisampleState,
    PipelineCompilationOptions, PipelineLayoutDescriptor, PrimitiveState, RenderPipelineDescriptor,
    TextureFormat, VertexState,
};

use crate::gpu::manager::ResourceManager;

#[derive(Default)]
pub struct RenderableScene {
    pub items: Vec<RenderableItem>, // other things will go here as well. lights, shader defines for scene
                                    // background and environments
}

impl RenderableScene {
    /// Extract renderable items in scene
    pub fn prepare(
        &mut self,
        scene: &Scene,
        assets: &AssetServer,
        resources: &mut ResourceManager,
    ) {
        for node in scene.nodes() {
            if let Some(mesh) = scene.get_mesh(node) {
                let mesh_geo = assets.geometry.get(mesh.geometry).unwrap();
                let geo_handle = resources.add_geometry(&mesh_geo);
                let mat_handle = resources.add_material(mesh.material, assets);

                let geometry = resources.geometry.get(geo_handle).unwrap();
                let material = resources.material.get(mat_handle).unwrap();
                let Some(frame) = resources.frame_globals.as_ref() else {
                    log::warn!("frame globals not ready");
                    return;
                };

                // TODO: Move to command and command generation
                let pipeline_layout =
                    resources
                        .device
                        .create_pipeline_layout(&PipelineLayoutDescriptor {
                            label: None,
                            bind_group_layouts: &[Some(&frame.layout), Some(material.layout())],
                            immediate_size: 0,
                        });

                let pipeline = resources
                    .device
                    .create_render_pipeline(&RenderPipelineDescriptor {
                        label: Some("renderer pipeline"),
                        layout: Some(&pipeline_layout),
                        vertex: VertexState {
                            module: &resources.shader,
                            entry_point: Some("vs_main"),
                            compilation_options: Default::default(),
                            buffers: &geometry.vertex_layout().wgpu_layout(),
                        },
                        primitive: PrimitiveState {
                            topology: wgpu::PrimitiveTopology::TriangleList, // TODO: Get from model
                            cull_mode: Some(Face::Back),
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
                            module: &resources.shader,
                            entry_point: Some("fs_main"),
                            compilation_options: PipelineCompilationOptions::default(),
                            targets: &[Some(ColorTargetState {
                                format: TextureFormat::Rgba8UnormSrgb, // self.config.color_format,
                                blend: None,
                                write_mask: ColorWrites::all(),
                            })],
                        }),
                        multiview_mask: None,
                        cache: None,
                    });

                resources.pipelines.insert(geo_handle, pipeline);

                self.items.push(RenderableItem {
                    geometry: geo_handle,
                    material: mat_handle,
                });
            }
        }
    }
}

#[derive(Clone)]
pub struct RenderableItem {
    pub geometry: GeometryHandle,
    pub material: MaterialHandle,
}
