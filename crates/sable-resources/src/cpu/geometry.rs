use std::collections::{HashMap, hash_map::Iter};

use bytemuck::{Pod, cast_ref, cast_slice};
use glam::Vec3;
use wgpu::{IndexFormat, PrimitiveTopology, VertexFormat};

pub use bounding::BoundingBox;

#[derive(Default, Clone)]
pub struct Geometry {
    attributes: HashMap<AttributeKind, Attribute>,
    index_attribute: Option<IndexAttribute>,
    pub vertex_count: usize,
    pub topology: PrimitiveTopology,
    bounding_box: BoundingBox,
}

impl Geometry {
    pub fn attributes(&self) -> Iter<'_, AttributeKind, Attribute> {
        self.attributes.iter()
    }

    pub fn set_attribute(&mut self, attribute: Attribute, kind: AttributeKind) {
        self.attributes.insert(kind, attribute);

        if kind == AttributeKind::Position {
            self.compute_bounding_volume();
        }
    }

    pub fn get_attribute(&self, kind: AttributeKind) -> Option<&Attribute> {
        self.attributes.get(&kind)
    }

    pub fn set_index_attribute(&mut self, attribute: Option<IndexAttribute>) {
        self.index_attribute = attribute;
    }

    pub fn get_index_attribute(&self) -> Option<&IndexAttribute> {
        self.index_attribute.as_ref()
    }

    pub fn bounding_box(&self) -> BoundingBox {
        self.bounding_box
    }

    fn compute_bounding_volume(&mut self) {
        let Some(positions) = self.attributes.get(&AttributeKind::Position) else {
            return;
        };

        if positions.format != VertexFormat::Float32x3 {
            return;
        }

        let offset = positions.offset as usize;
        let stride = positions.stride as usize;
        let count = positions.count as usize;

        let mut min = Vec3::splat(f32::INFINITY);
        let mut max = Vec3::splat(f32::NEG_INFINITY);
        let mut valid_points = 0;

        for i in 0..count {
            let start = offset + i * stride;
            let end = start + size_of::<[f32; 3]>();

            if let Some(slice) = positions.data.get(start..end) {
                if let Ok(bytes) = slice.try_into() as Result<&[u8; 12], _> {
                    let vals = cast_ref(bytes);
                    let vec = Vec3::from_array(*vals);
                    min = vec.min(min);
                    max = vec.max(max);

                    valid_points += 1;
                }
            } else {
                break;
            }
        }
        if valid_points == 0 {
            return;
        }
        self.bounding_box = BoundingBox { min, max };
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AttributeKind {
    Position,
    Color,
    Normal,
    TexCoords,
}

impl AttributeKind {
    pub fn shader_location(&self) -> u32 {
        match self {
            Self::Position => 0,
            Self::Color => 1,
            Self::Normal => 2,
            Self::TexCoords => 3,
        }
    }
}

#[allow(dead_code)]
#[derive(Clone)]
pub struct Attribute {
    data: Vec<u8>,
    pub format: VertexFormat,
    pub offset: u64,
    pub stride: u64,
    pub count: u32,
}

impl Attribute {
    pub fn new<T: Pod>(data: &[T], format: VertexFormat) -> Self {
        let bytes = cast_slice(data).to_vec();

        Self {
            data: bytes,
            format,
            offset: 0,
            stride: size_of::<T>() as u64,
            count: data.len() as u32,
        }
    }

    pub fn data(&self) -> &[u8] {
        self.data.as_slice()
    }
}

#[derive(Clone)]
pub struct IndexAttribute {
    data: Vec<u8>,
    pub format: IndexFormat,
    pub count: u32,
}

impl IndexAttribute {
    pub fn new<T: Pod>(data: &[T], format: IndexFormat) -> Self {
        let bytes = cast_slice(data).to_vec();

        Self {
            data: bytes,
            format,
            count: data.len() as u32,
        }
    }

    pub fn data(&self) -> &[u8] {
        self.data.as_slice()
    }
}

mod bounding {
    use glam::{Affine3A, Vec3, vec3};

    #[derive(Debug, Default, Clone, Copy)]
    pub struct BoundingBox {
        pub min: Vec3,
        pub max: Vec3,
    }

    impl BoundingBox {
        pub fn center(&self) -> Vec3 {
            (self.min + self.max) / 2.0
        }

        pub fn size(&self) -> Vec3 {
            self.max - self.min
        }

        pub fn transform(&self, transform: &Affine3A) -> Self {
            // - list of corners
            let corners = [
                vec3(self.min.x, self.min.y, self.min.z),
                vec3(self.min.x, self.max.y, self.min.z),
                vec3(self.min.x, self.min.y, self.max.z),
                vec3(self.min.x, self.max.y, self.max.z),
                vec3(self.max.x, self.max.y, self.max.z),
                vec3(self.max.x, self.min.y, self.max.z),
                vec3(self.max.x, self.max.y, self.min.z),
                vec3(self.max.x, self.min.y, self.min.z),
            ];

            // - new min/max at extremities
            let mut new_min = Vec3::splat(f32::INFINITY);
            let mut new_max = Vec3::splat(f32::NEG_INFINITY);

            // - transform each corner with matrix
            for corner in corners {
                let point = transform.transform_point3(corner);

                // - compare with new_min/max
                new_min = new_min.min(point);
                new_max = new_max.max(point);
            }

            // - return new_min/max as box
            Self {
                min: new_min,
                max: new_max,
            }
        }

        pub fn union(&self, other: &Self) -> Self {
            Self {
                min: self.min.min(other.min),
                max: self.max.max(other.max),
            }
        }
    }
}
