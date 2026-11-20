use std::collections::HashSet;

use glam::Vec2;

#[derive(Debug, Default)]
pub struct InputState {
    pressed_keys: HashSet<Key>,
    just_pressed_key: HashSet<Key>,
    just_released_key: HashSet<Key>,

    pressed_mouse: HashSet<MouseButton>,
    just_pressed_mouse: HashSet<MouseButton>,
    just_released_mouse: HashSet<MouseButton>,

    cursor_position: Vec2,
    cursor_delta: Vec2,
    scroll_delta: Vec2,

    screen_size: Vec2,
}

impl InputState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn start_frame(&mut self) {
        self.just_pressed_key.clear();
        self.just_released_key.clear();
        self.just_pressed_mouse.clear();
        self.just_released_mouse.clear();

        self.cursor_delta = Vec2::ZERO;
        self.scroll_delta = Vec2::ZERO;
    }

    pub fn handle_key_press(&mut self, key: Key, state: KeyState) {
        match state {
            KeyState::Pressed => {
                if self.pressed_keys.insert(key) {
                    self.just_pressed_key.insert(key);
                }
            }
            KeyState::Released => {
                if self.pressed_keys.remove(&key) {
                    self.just_released_key.insert(key);
                }
            }
        }
    }

    pub fn handle_mouse_press(&mut self, button: MouseButton, state: ButtonState) {
        match state {
            ButtonState::Pressed => {
                if self.pressed_mouse.insert(button) {
                    self.just_pressed_mouse.insert(button);
                }
            }
            ButtonState::Released => {
                if self.pressed_mouse.remove(&button) {
                    self.just_released_mouse.insert(button);
                }
            }
        }
    }

    pub fn handle_mouse_move(&mut self, new_x: f32, new_y: f32) {
        let new_pos = Vec2::new(new_x, new_y);

        if self.cursor_position != Vec2::ZERO {
            self.cursor_delta += new_pos - self.cursor_position;
        }

        self.cursor_position = new_pos;
    }

    pub fn handle_scroll(&mut self, delta_x: f32, delta_y: f32) {
        self.scroll_delta = Vec2::new(delta_x, delta_y);
    }

    pub fn handle_resize(&mut self, width: u32, height: u32) {
        self.screen_size = Vec2::new(width as f32, height as f32);
    }

    pub fn is_mouse_down(&self, button: MouseButton) -> bool {
        self.pressed_mouse.contains(&button)
    }

    pub fn mouse_delta(&self) -> Vec2 {
        self.cursor_delta
    }

    pub fn scroll_delta(&self) -> Vec2 {
        self.scroll_delta
    }

    pub fn screen_size(&self) -> Vec2 {
        self.screen_size
    }
}

/// Keyboard key enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Key {
    // Letter keys
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,

    // Number keys
    Key0,
    Key1,
    Key2,
    Key3,
    Key4,
    Key5,
    Key6,
    Key7,
    Key8,
    Key9,

    // Function keys
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,

    // Control keys
    Space,
    Enter,
    Escape,
    Backspace,
    Tab,
    Delete,
    Insert,
    Home,
    End,
    PageUp,
    PageDown,

    // Modifier keys
    ShiftLeft,
    ShiftRight,
    ControlLeft,
    ControlRight,
    AltLeft,
    AltRight,
    SuperLeft,
    SuperRight,

    // Arrow keys
    ArrowUp,
    ArrowDown,
    ArrowLeft,
    ArrowRight,

    // Numpad
    Numpad0,
    Numpad1,
    Numpad2,
    Numpad3,
    Numpad4,
    Numpad5,
    Numpad6,
    Numpad7,
    Numpad8,
    Numpad9,
    NumpadEnter,
}

/// Button state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyState {
    Pressed,
    Released,
}

/// Mouse button enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
}

/// Button state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ButtonState {
    Pressed,
    Released,
}
