use std::collections::{HashMap, hash_map::Iter};

use bytemuck::{Pod, cast_slice};
use wgpu::VertexFormat;

#[derive(Default)]
pub struct Mesh {
    pub(crate) geometry: Geometry,
}

#[derive(Default)]
pub struct Geometry {
    attributes: HashMap<AttributeKind, Attribute>,
}

impl Geometry {
    pub fn attributes(&self) -> Iter<'_, AttributeKind, Attribute> {
        self.attributes.iter()
    }

    pub fn set_attribute(&mut self, attribute: Attribute, kind: AttributeKind) {
        self.attributes.insert(kind, attribute);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AttributeKind {
    Position,
    Color,
}

impl AttributeKind {
    pub fn shader_location(&self) -> u32 {
        match self {
            Self::Position => 0,
            Self::Color => 1,
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
}

impl Attribute {
    pub fn new<T: Pod>(data: &[T], format: VertexFormat) -> Self {
        let data = cast_slice(data).to_vec();

        Self {
            data,
            format,
            offset: 0,
            stride: size_of::<T>() as u64,
        }
    }

    pub fn data(&self) -> &[u8] {
        self.data.as_slice()
    }
}
