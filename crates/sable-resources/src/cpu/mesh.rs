use sable_core::handle::GeometryHandle;

use crate::{Geometry, Material};

#[derive(Default)]
pub struct Mesh {
    pub geometry: Geometry,
    pub geometry_handle: GeometryHandle,
    // pub material: Material,
}
