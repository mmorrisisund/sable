use std::time::{Duration, Instant};

use gpui::{
    Application, Bounds, KeyBinding, Point, Size, WgpuSurfaceHandle, Window, WindowBounds,
    WindowOptions, actions, div, prelude::*, px, rgb, wgpu_surface,
};
use wgpu::{
    Color, ColorTargetState, ColorWrites, Device, FragmentState, LoadOp, MultisampleState,
    Operations, PipelineCompilationOptions, PipelineLayoutDescriptor, PrimitiveState, Queue,
    RenderPassColorAttachment, RenderPassDescriptor, RenderPipeline, RenderPipelineDescriptor,
    ShaderModuleDescriptor, ShaderSource, StoreOp, TextureFormat, TextureView, VertexState,
};

fn main() {
    env_logger::init();

    Application::new().run(|cx| {
        cx.activate(true);
        cx.on_action(|_: &Quit, cx| cx.quit());
        cx.bind_keys([KeyBinding::new("escape", Quit, None)]);

        let _ = cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(Bounds::new(
                    Point::new(px(10.0), px(10.0)),
                    Size::new(px(1000.0), px(800.0)),
                ))),
                ..Default::default()
            },
            |window, cx| {
                let surface = window
                    .create_wgpu_surface(1000, 100, wgpu::TextureFormat::Rgba8UnormSrgb)
                    .expect("wgpu not supported on this machine");

                let now = Instant::now();
                cx.new(|_| SimpleApp {
                    surface,
                    frame_count: 0,
                    last_fps_update: now,
                    display_fps: 0.0,
                    state: None,
                })
            },
        );
    })
}

actions!(editor, [Quit]);

struct SimpleApp {
    surface: WgpuSurfaceHandle,
    frame_count: u32,
    last_fps_update: Instant,
    display_fps: f64,
    state: Option<AppState>,
}

impl Render for SimpleApp {
    fn render(&mut self, window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        if let Some((view, (w, h))) = self.surface.back_view_with_size() {
            let state = self.state.get_or_insert_with(|| {
                let device = self.surface.device().clone();
                let queue = self.surface.queue().clone();
                AppState::new(device, queue, w, h, self.surface.format())
            });

            state.render(&view);
            drop(view);

            self.surface.swap_buffers();

            let now = Instant::now();

            self.frame_count = self.frame_count.wrapping_add(1);
            if now.duration_since(self.last_fps_update) >= Duration::from_secs(1) {
                self.display_fps = self.frame_count as f64;
                self.frame_count = 0;
                self.last_fps_update = now;
            }

            window.request_animation_frame();
        }

        div()
            .size_full()
            .flex()
            .flex_row()
            .child(
                div()
                    .w(px(232.0))
                    .text_color(rgb(0x00aaff))
                    .child("Details Pane"),
            )
            .child(
                div()
                    .size_full()
                    .border_4()
                    .rounded_lg()
                    .shadow_xl()
                    .bg(rgb(0x000000))
                    // .m(px(8.0))
                    .child(wgpu_surface(self.surface.clone()).absolute().inset_0())
                    .child(
                        div()
                            .absolute()
                            .top(px(4.0))
                            .left(px(8.0))
                            .text_color(rgb(0x00aaff))
                            .text_xl()
                            .child(format!("FPS: {:.1} | SimpleApp", self.display_fps)),
                    ),
            )
    }
}

struct AppState {
    device: Device,
    queue: Queue,
    pipeline: RenderPipeline,
}

impl AppState {
    fn new(device: Device, queue: Queue, _w: u32, _h: u32, color_format: TextureFormat) -> Self {
        let shader = device.create_shader_module(ShaderModuleDescriptor {
            label: None,
            source: ShaderSource::Wgsl(SHADER.into()),
        });

        let layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[],
            immediate_size: 0,
        });

        let pipeline = device.create_render_pipeline(&RenderPipelineDescriptor {
            label: None,
            layout: Some(&layout),
            vertex: VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                compilation_options: Default::default(),
                buffers: &[],
            },
            primitive: PrimitiveState::default(),
            depth_stencil: None,
            multisample: MultisampleState::default(),
            fragment: Some(FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                compilation_options: PipelineCompilationOptions::default(),
                targets: &[Some(ColorTargetState {
                    format: color_format,
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
            pipeline,
        }
    }

    fn render(&self, view: &TextureView) {
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
            rp.draw(0..3, 0..1);
        }

        self.queue.submit(Some(encoder.finish()));
    }
}

const SHADER: &str = r#"
    @vertex
    fn vs_main() -> @builtin(position) vec4f {
        return vec4f(0.0);
    }

    @fragment
    fn fs_main() -> @location(0) vec4f {
        return vec4f(0.0);
    }
"#;
