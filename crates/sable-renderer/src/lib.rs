mod camera;
pub(crate) mod gpu;
mod renderer;
pub mod settings;

pub use camera::{CameraUniform, RenderCamera};
pub use renderer::{Renderer, RendererConfig};

use gpu::{geometry::GpuGeometry, manager::ResourceManager, material::GpuMaterial};
