use std::time::{Duration, Instant};

use gpui::{
    Application, Bounds, KeyBinding, MouseButton, MouseDownEvent, MouseMoveEvent, Pixels, Point,
    Size, WgpuSurfaceHandle, Window, WindowBounds, WindowOptions, actions, div, prelude::*, px,
    rgb, wgpu_surface,
};
use sable::{Renderer, RendererConfig};
use sable_assets::AssetServer;
use wgpu::{Device, Queue, TextureFormat, TextureView};

const ASSET_ROOT: &str = r#"C:\Users\mike\Documents\dev\rust\graphics\glTF-Sample-Assets\"#;

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
                cx.new(|_| ViewerApp {
                    surface,
                    frame_count: 0,
                    last_fps_update: now,
                    display_fps: 0.0,
                    state: None,

                    pos_x: Pixels::ZERO,
                    pos_y: Pixels::ZERO,
                    mouse_down: false,
                    dragging: false,
                    button: None,
                })
            },
        );
    })
}

actions!(editor, [Quit]);

struct ViewerApp {
    surface: WgpuSurfaceHandle,
    frame_count: u32,
    last_fps_update: Instant,
    display_fps: f64,
    state: Option<AppState>,

    // mouse state
    pos_x: Pixels,
    pos_y: Pixels,
    mouse_down: bool,
    dragging: bool,
    button: Option<MouseButton>,
}

impl ViewerApp {
    fn details_pane(&mut self) -> impl IntoElement {
        div().w(px(232.0)).text_color(rgb(0x00aaff)).child(
            div()
                .flex()
                .flex_col()
                .child(
                    div()
                        .flex()
                        .flex_row()
                        .child("Position X: ")
                        .child(format!("{:.0}", self.pos_x.value())),
                )
                .child(
                    div()
                        .flex()
                        .flex_row()
                        .child("Position Y: ")
                        .child(format!("{:.0}", self.pos_y.value())),
                )
                .child(
                    div()
                        .flex()
                        .flex_row()
                        .child(format!("Dragging: {}", self.dragging)),
                )
                .child(
                    div()
                        .flex()
                        .flex_row()
                        .child(format!("Mouse Down: {}", self.mouse_down)),
                )
                .child(
                    div()
                        .flex()
                        .flex_row()
                        .child(format!("Button: {:?}", self.button)),
                ),
        )
    }
}

impl Render for ViewerApp {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
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
                window.set_window_title(&format!("FPS: {:.1} | glTF Viewer", self.display_fps));
            }

            window.request_animation_frame();
        }

        div()
            .size_full()
            .flex()
            .flex_row()
            .child(self.details_pane())
            .child(
                div()
                    .size_full()
                    .border_4()
                    .rounded_lg()
                    .shadow_xl()
                    .bg(rgb(0x000000))
                    .on_any_mouse_down(cx.listener(|this, e: &MouseDownEvent, _, _| {
                        this.mouse_down = true;
                        this.button = Some(e.button);
                    }))
                    .capture_any_mouse_up(cx.listener(|this, _, _, _| {
                        this.mouse_down = false;
                        this.button = None;
                    }))
                    .on_mouse_move(cx.listener(|this, e: &MouseMoveEvent, _: &mut Window, _| {
                        this.pos_x = e.position.x;
                        this.pos_y = e.position.y;
                        this.dragging = this.mouse_down;
                    }))
                    .child(wgpu_surface(self.surface.clone()).absolute().inset_0()),
            )
    }
}

#[allow(dead_code)]
struct AppState {
    device: Device,
    queue: Queue,
    renderer: Renderer,
}

impl AppState {
    fn new(device: Device, queue: Queue, w: u32, h: u32, color_format: TextureFormat) -> Self {
        let config = RendererConfig {
            width: w,
            height: h,
            color_format,
        };
        let mut renderer = Renderer::new(device.clone(), queue.clone(), config);

        let model = std::env::args().nth(1).unwrap();
        let path = format!("{ASSET_ROOT}/{model}/glTF/{model}.gltf");

        let mut server = AssetServer::new();
        let prefab = server.load_model(&path).unwrap();
        renderer.add_prefab(prefab, &server);

        Self {
            device,
            queue,
            renderer,
        }
    }

    fn render(&self, view: &TextureView) {
        use sable::renderer::Camera;
        self.renderer.render(&Camera::default(), view);
    }
}
