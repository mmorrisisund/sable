use std::time::{Duration, Instant};

use glam::{Vec3, vec2, vec3};
use gpui::{
    Application, Bounds, KeyBinding, MouseButton, MouseDownEvent, MouseMoveEvent, MouseUpEvent,
    Pixels, Point, ScrollDelta, ScrollWheelEvent, Size, WgpuSurfaceHandle, Window, WindowBounds,
    WindowOptions, actions, div, prelude::*, px, rgb, wgpu_surface,
};
use sable::{Renderer, RendererConfig};
use sable_assets::{AssetServer, scene_ext::SceneExt};
use sable_resources::Scene;
use wgpu::{Device, Queue, TextureFormat, TextureView};

use crate::camera::OrbitCamera;

const ASSET_ROOT: &str = r#"C:\Users\mike\code\assets\glTF\"#;

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
                cx.new(|_| ViewerApp {
                    surface,
                    frame_count: 0,
                    last_fps_update: now,
                    display_fps: 0.0,
                    renderer: None,

                    state: AppState::default(),
                })
            },
        );
    })
}

actions!(editor, [Quit]);

#[derive(Default)]
struct AppState {
    // mouse state
    position: Point<Pixels>,
    mouse_down: bool,
    dragging: bool,
    button: Option<MouseButton>,
}

struct ViewerApp {
    surface: WgpuSurfaceHandle,
    frame_count: u32,
    last_fps_update: Instant,
    display_fps: f64,
    state: AppState,
    renderer: Option<RenderState>,
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
                        .child(format!("{:.0}", self.state.position.x.value())),
                )
                .child(
                    div()
                        .flex()
                        .flex_row()
                        .child("Position Y: ")
                        .child(format!("{:.0}", self.state.position.y.value())),
                )
                .child(
                    div()
                        .flex()
                        .flex_row()
                        .child(format!("Dragging: {}", self.state.dragging)),
                )
                .child(
                    div()
                        .flex()
                        .flex_row()
                        .child(format!("Mouse Down: {}", self.state.mouse_down)),
                )
                .child(
                    div()
                        .flex()
                        .flex_row()
                        .child(format!("Button: {:?}", self.state.button)),
                ),
        )
    }
}

impl Render for ViewerApp {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        if let Some((view, (w, h))) = self.surface.back_view_with_size() {
            let renderer = self.renderer.get_or_insert_with(|| {
                let device = self.surface.device().clone();
                let queue = self.surface.queue().clone();

                RenderState::new(device, queue, w, h, self.surface.format())
            });

            // "tick" state here

            if renderer.width != w || renderer.height != h {
                renderer.resize(w, h);
            }

            renderer.render(&view);
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
                    .on_any_mouse_down(cx.listener(Self::handle_any_mouse_down))
                    .capture_any_mouse_up(cx.listener(Self::handle_mouse_up))
                    .on_mouse_move(cx.listener(Self::handle_mouse_move))
                    .on_scroll_wheel(cx.listener(Self::handle_scroll))
                    .child(wgpu_surface(self.surface.clone()).absolute().inset_0()),
            )
    }
}

impl ViewerApp {
    fn handle_mouse_up(
        this: &mut Self,
        _e: &MouseUpEvent,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) {
        this.state.mouse_down = false;
        this.state.button = None;
    }

    fn handle_any_mouse_down(
        this: &mut Self,
        e: &MouseDownEvent,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) {
        this.state.mouse_down = true;
        this.state.button = Some(e.button);
    }

    fn handle_mouse_move(
        &mut self,
        e: &MouseMoveEvent,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) {
        let Some(state) = self.renderer.as_mut() else {
            return;
        };

        let delta = e.position - self.state.position;
        self.state.position = e.position;

        if self.state.mouse_down {
            state.camera.rotate(vec2(delta.x.into(), delta.y.into()));
        }
    }

    fn handle_scroll(
        &mut self,
        e: &ScrollWheelEvent,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) {
        let Some(state) = self.renderer.as_mut() else {
            return;
        };

        let delta = e.delta.pixel_delta(Pixels::from(100.0f32));

        state.camera.zoom(delta.y.value());
    }
}

#[allow(dead_code)]
struct RenderState {
    device: Device,
    queue: Queue,
    renderer: Renderer,
    camera: OrbitCamera,
    width: u32,
    height: u32,
}

impl RenderState {
    fn new(device: Device, queue: Queue, w: u32, h: u32, color_format: TextureFormat) -> Self {
        let config = RendererConfig {
            width: w,
            height: h,
            color_format,
        };

        let model = std::env::args().nth(1).unwrap();
        let path = format!("{ASSET_ROOT}/{model}/glTF/{model}.gltf");

        let mut server = AssetServer::new();
        let prefab_handle = server.load_model(&path).unwrap();
        let prefab = server.prefab.get(prefab_handle).unwrap();

        let mut renderer = Renderer::new(device.clone(), queue.clone(), config, server);

        let mut scene = Scene::new();
        scene.instantiate_prefab(&prefab);
        renderer.set_scene(scene);

        // renderer.add_prefab(prefab_handle);

        let camera = OrbitCamera::new(
            vec3(0.0, 0.0, 5.0),
            Vec3::ZERO,
            45.0f32.to_radians(),
            w as f32 / h as f32,
            0.01,
            100.0,
        );

        Self {
            device,
            queue,
            renderer,
            camera,
            width: w,
            height: h,
        }
    }

    fn render(&mut self, view: &TextureView) {
        self.renderer.start_frame();
        self.renderer.render(&self.camera, view);
    }

    fn resize(&mut self, w: u32, h: u32) {
        self.width = w;
        self.height = h;
        self.renderer.resize(w, h);
    }
}

mod camera {
    use glam::{
        Mat4, Vec2, Vec3,
        camera::rh::{proj, view},
    };
    use sable::renderer::{CameraUniform, RenderCamera};

    pub struct OrbitCamera {
        position: Vec3,
        target: Vec3,
        projection: Mat4,
        view: Mat4,

        radius: f32,
        yaw: f32,
        pitch: f32,
        sensitivity: f32,
    }

    impl OrbitCamera {
        pub fn new(
            position: Vec3,
            target: Vec3,
            fov_y_radians: f32,
            aspect: f32,
            near: f32,
            far: f32,
        ) -> Self {
            Self {
                position,
                target,
                projection: proj::directx::perspective(fov_y_radians, aspect, near, far),
                view: view::look_at_mat4(position, target, Vec3::Y),

                radius: 3.0,
                yaw: 0.0,
                pitch: 0.0,
                sensitivity: 0.005,
            }
        }

        pub fn zoom(&mut self, delta: f32) {
            self.radius = (self.radius + delta * self.sensitivity).max(0.0);
            let dir = Vec3::new(
                self.yaw.cos() * self.pitch.cos(),
                self.pitch.sin(),
                self.yaw.sin() * self.pitch.cos(),
            )
            .normalize();

            self.position = self.target - dir * self.radius;
            self.view = view::look_at_mat4(self.position, self.target, Vec3::Y);
        }

        pub fn rotate(&mut self, delta: Vec2) {
            self.yaw += delta.x * self.sensitivity;
            self.pitch = (self.pitch - delta.y * self.sensitivity).clamp(-1.5, 1.5);

            let dir = Vec3::new(
                self.yaw.cos() * self.pitch.cos(),
                self.pitch.sin(),
                self.yaw.sin() * self.pitch.cos(),
            )
            .normalize();

            self.position = self.target - dir * self.radius;
            self.view = view::look_at_mat4(self.position, self.target, Vec3::Y);
        }
    }

    impl RenderCamera for OrbitCamera {
        fn extract_uniform(&self) -> CameraUniform {
            CameraUniform {
                view: self.view,
                projection: self.projection,
                view_projection: self.projection * self.view,
                position: self.position,
                ..Default::default()
            }
        }
    }
}
