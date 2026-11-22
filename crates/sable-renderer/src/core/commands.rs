pub mod render {
    use std::ops::Range;

    use wgpu::{BindGroup, Buffer, IndexFormat, RenderPipeline};

    use crate::{core::scene::RenderableItem, gpu::manager::ResourceManager};

    pub struct RenderCommands<'a> {
        pub opaque: Vec<RenderCommand<'a>>,
        pub frame_bg: BindGroup,
    }

    impl<'a> RenderCommands<'a> {
        pub fn generate(
            renderables: &[RenderableItem],
            resources: &'a mut ResourceManager,
        ) -> Self {
            let frame_bg = resources
                .frame_globals
                .as_ref()
                .expect("frame binding must be created")
                .bindgroup
                .clone();
            let opaque = Vec::new();

            let mut me = Self { frame_bg, opaque };
            me.generate_opaque(renderables, resources);

            me
        }

        fn generate_opaque(
            &mut self,
            renderables: &[RenderableItem],
            resources: &'a mut ResourceManager,
        ) {
            for item in renderables {
                let pipeline = resources.pipelines.get(item.geometry).unwrap();
                let material_bg = resources.material.get(item.material).unwrap().bind_group();
                let vertex_buffers = resources
                    .geometry
                    .get(item.geometry)
                    .unwrap()
                    .vertex_buffers()
                    .to_vec();
                let index = if let Some(gpu_geo) = resources.geometry.get(item.geometry)
                    && let Some(buffer) = gpu_geo.index_buffer()
                {
                    let count = gpu_geo.count();
                    let format = *gpu_geo.index_format().unwrap();

                    Some(IndexCommand {
                        buffer,
                        count,
                        format,
                    })
                } else {
                    None
                };

                self.opaque.push(RenderCommand {
                    pipeline,
                    material_bg,
                    vertex_buffers,
                    index,
                });
            }
        }
    }

    pub struct RenderCommand<'a> {
        pub pipeline: &'a RenderPipeline,
        pub material_bg: &'a BindGroup,
        pub vertex_buffers: Vec<Buffer>,
        index: Option<IndexCommand<'a>>,
    }

    impl<'a> RenderCommand<'a> {
        pub fn vertex_buffers(&self) -> &[Buffer] {
            &self.vertex_buffers
        }

        pub fn index_buffer(&self) -> Option<(&Buffer, u32, IndexFormat)> {
            self.index
                .as_ref()
                .map(|cmd| (cmd.buffer, cmd.count, cmd.format))
        }

        pub fn vertex_range(&self) -> Range<u32> {
            todo!()
        }

        pub fn instance_range(&self) -> Range<u32> {
            0..1
        }
    }

    struct IndexCommand<'a> {
        buffer: &'a Buffer,
        count: u32,
        format: IndexFormat,
    }
}
