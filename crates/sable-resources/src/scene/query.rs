use sable_core::handle::GeometryHandle;

use crate::geometry::BoundingBox;

/// Implemented by AssetServer to get bounding box of geometry
/// without needing a reference to the asset crate
pub trait Query {
    fn get_geometry_bounding_box(&self, handle: GeometryHandle) -> Option<BoundingBox>;
}
