use crate::{Geometry, Material};

#[derive(Default)]
pub struct Mesh {
    pub geometry: Geometry,
    pub material: Material,
}
