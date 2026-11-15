use std::time::{Duration, Instant};

use gpui::{
    Application, Bounds, KeyBinding, MouseButton, MouseDownEvent, MouseMoveEvent, MouseUpEvent,
    Pixels, Point, ScrollWheelEvent, Size, WgpuSurfaceHandle, Window, WindowBounds, WindowOptions,
    actions, div, prelude::*, px, rgb, wgpu_surface,
};
use sable::Engine;
use sable_assets::scene_ext::SceneExt;
use sable_core::math::Deg;
use sable_resources::{Camera, Scene};

const ASSET_ROOT: &str = r#"C:\Users\mike\code\assets\glTF\"#;
const DEFAULT_WIDTH: f32 = 1000.0;
const DEFAULT_HEIGHT: f32 = 800.0;

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
                    Size::new(px(DEFAULT_WIDTH), px(DEFAULT_HEIGHT)),
                ))),
                ..Default::default()
            },
            |window, cx| {
                let surface = window
                    .create_wgpu_surface(
                        DEFAULT_WIDTH as u32,
                        DEFAULT_HEIGHT as u32,
                        wgpu::TextureFormat::Rgba8UnormSrgb,
                    )
                    .expect("wgpu not supported on this machine");

                cx.new(|_| ViewerApp::new(surface))
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
    engine: Engine,
    size: (u32, u32),
}

impl ViewerApp {
    fn new(surface: WgpuSurfaceHandle) -> Self {
        let model = std::env::args().nth(1).unwrap();
        let model_path = format!("{ASSET_ROOT}/{model}/glTF/{model}.gltf");

        let size = surface.size();
        let mut engine = Engine::new(surface.device().clone(), surface.queue().clone(), size);

        let prefab_handle = engine.assets.load_model(&model_path).unwrap();
        let prefab = engine.assets.prefab.get(prefab_handle).unwrap();

        let mut scene = Scene::new();

        let camera =
            Camera::new_perspective(Deg::new(45.0), DEFAULT_WIDTH / DEFAULT_HEIGHT, 0.1, 1000.0);

        scene.add_active_camera(camera);
        scene.instantiate_prefab(&prefab);

        engine.scenes.add_active_scene(scene);

        Self {
            surface,
            frame_count: 0,
            last_fps_update: Instant::now(),
            display_fps: 0.0,

            state: AppState::default(),
            engine,
            size,
        }
    }

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

    fn did_resize(&mut self, size: (u32, u32)) -> bool {
        let changed = self.size.0 != size.0 || self.size.1 != size.1;
        if changed {
            self.size = size;
        }
        changed
    }
}

impl Render for ViewerApp {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        if let Some((view, (w, h))) = self.surface.back_view_with_size() {
            if self.did_resize((w, h)) {
                self.engine.resize(w, h);
            }

            // "tick" state here
            self.engine.render_frame(&view);
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
        this.engine.input.handle_mouse_press(
            sable_app::MouseButton::Left,
            sable_app::ButtonState::Released,
        );
    }

    fn handle_any_mouse_down(
        this: &mut Self,
        _e: &MouseDownEvent,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) {
        this.engine.input.handle_mouse_press(
            sable_app::MouseButton::Left,
            sable_app::ButtonState::Pressed,
        );
    }

    fn handle_mouse_move(
        &mut self,
        e: &MouseMoveEvent,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) {
        self.engine
            .input
            .handle_mouse_move(e.position.x.to_f32(), e.position.y.to_f32());

        // let delta = e.position - self.state.position;
        self.state.position = e.position;

        if self.state.mouse_down {
            // self.camera.rotate(vec2(delta.x.into(), delta.y.into()));
        }
    }

    fn handle_scroll(
        &mut self,
        _e: &ScrollWheelEvent,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) {
        // let delta = e.delta.pixel_delta(Pixels::from(100.0f32));

        // self.camera.zoom(delta.y.value());
    }
}

// mod camera {
//     use glam::{
//         Mat4, Vec2, Vec3,
//         camera::rh::{proj, view},
//     };
//     use sable::renderer::{CameraUniform, RenderCamera};

//     pub struct OrbitCamera {
//         position: Vec3,
//         target: Vec3,
//         projection: Mat4,
//         view: Mat4,

//         radius: f32,
//         yaw: f32,
//         pitch: f32,
//         sensitivity: f32,
//     }

//     impl OrbitCamera {
//         pub fn new(
//             position: Vec3,
//             target: Vec3,
//             fov_y_radians: f32,
//             aspect: f32,
//             near: f32,
//             far: f32,
//         ) -> Self {
//             Self {
//                 position,
//                 target,
//                 projection: proj::directx::perspective(fov_y_radians, aspect, near, far),
//                 view: view::look_at_mat4(position, target, Vec3::Y),

//                 radius: target.distance(position),
//                 yaw: 0.0,
//                 pitch: 0.0,
//                 sensitivity: 0.005,
//             }
//         }

//         pub fn zoom(&mut self, delta: f32) {
//             self.radius = (self.radius + delta * self.sensitivity).max(0.0);
//             let dir = Vec3::new(
//                 self.yaw.sin() * self.pitch.cos(),
//                 self.pitch.sin(),
//                 self.yaw.cos() * self.pitch.cos(),
//             )
//             .normalize();

//             self.position = self.target - dir * -self.radius;
//             self.view = view::look_at_mat4(self.position, self.target, Vec3::Y);
//         }

//         pub fn rotate(&mut self, delta: Vec2) {
//             self.yaw += delta.x * self.sensitivity;
//             self.pitch = (self.pitch + delta.y * self.sensitivity).clamp(-1.5, 1.5);

//             let dir = Vec3::new(
//                 self.yaw.cos() * self.pitch.cos(),
//                 self.pitch.sin(),
//                 self.yaw.sin() * self.pitch.cos(),
//             )
//             .normalize();

//             self.position = self.target - dir * -self.radius;
//             self.view = view::look_at_mat4(self.position, self.target, Vec3::Y);
//         }
//     }

//     impl RenderCamera for OrbitCamera {
//         fn extract_uniform(&self) -> CameraUniform {
//             CameraUniform {
//                 view: self.view,
//                 projection: self.projection,
//                 view_projection: self.projection * self.view,
//                 position: self.position,
//                 ..Default::default()
//             }
//         }
//     }
// }
