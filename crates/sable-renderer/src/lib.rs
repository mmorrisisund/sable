pub(crate) mod gpu;
mod renderer;
pub mod settings;

pub use renderer::{Renderer, RendererConfig};

use gpu::{geometry::GpuGeometry, manager::ResourceManager, material::GpuMaterial};
