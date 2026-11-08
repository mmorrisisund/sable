use sable_core::handle::{GeometryHandle, MaterialHandle};

#[derive(Debug, Default, Clone)]
pub struct Mesh {
    pub geometry: GeometryHandle,
    pub material: MaterialHandle,
}
