mod cpu;
mod gpu;

pub use cpu::{
    geometry::{self, Attribute, AttributeKind, Geometry, IndexAttribute},
    material::{self, Material},
    mesh::{self, Mesh},
    scene::Scene,
};
pub use gpu::{geometry::GpuGeometry, manager::ResourceManager};
