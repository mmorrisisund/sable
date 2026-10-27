use std::collections::{HashMap, hash_map::Iter};

use bytemuck::{Pod, cast_slice};
use wgpu::{IndexFormat, PrimitiveTopology, VertexFormat};

#[derive(Default, Clone)]
pub struct Geometry {
    attributes: HashMap<AttributeKind, Attribute>,
    index_attribute: Option<IndexAttribute>,
    pub vertex_count: usize,
    pub topology: PrimitiveTopology,
}

impl Geometry {
    pub fn attributes(&self) -> Iter<'_, AttributeKind, Attribute> {
        self.attributes.iter()
    }

    pub fn set_attribute(&mut self, attribute: Attribute, kind: AttributeKind) {
        self.attributes.insert(kind, attribute);
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
