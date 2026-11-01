use std::time::{Duration, Instant};

use gpui::{
    Application, Bounds, KeyBinding, Point, Size, WgpuSurfaceHandle, Window, WindowBounds,
    WindowOptions, actions, div, prelude::*, px, rgb, wgpu_surface,
};
use sable::{Renderer, RendererConfig, renderer::CameraUniform};
use wgpu::{Device, Queue, TextureFormat, TextureView};

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
                    .create_wgpu_surface(1000, 800, wgpu::TextureFormat::Rgba8UnormSrgb)
                    .expect("wgpu not supported on this machine");

                let now = Instant::now();
                cx.new(|_| TriangleApp {
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

struct TriangleApp {
    surface: WgpuSurfaceHandle,
    frame_count: u32,
    last_fps_update: Instant,
    display_fps: f64,
    state: Option<AppState>,
}

impl Render for TriangleApp {
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
                            .child(format!("FPS: {:.1} | TriangleApp", self.display_fps)),
                    ),
            )
    }
}

struct AppState {
    renderer: Renderer,
}

impl AppState {
    fn new(device: Device, queue: Queue, w: u32, h: u32, color_format: TextureFormat) -> Self {
        let config = RendererConfig {
            width: w,
            height: h,
            color_format,
        };
        let renderer = Renderer::new(device, queue, config);

        Self { renderer }
    }

    fn render(&self, view: &TextureView) {
        // self.renderer.render(&CameraUniform::default(), view);
    }
}
