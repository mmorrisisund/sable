use wgpu::{
    Buffer, BufferUsages, Device, IndexFormat, VertexAttribute, VertexBufferLayout, VertexStepMode,
    util::{BufferInitDescriptor, DeviceExt},
};

use crate::{AttributeKind, Geometry};

pub struct GpuGeometry {
    layout_info: GeneratedVertexLayout,
    buffers: Vec<Buffer>,
    index_buffer: Option<Buffer>,
    index_format: Option<IndexFormat>,
    count: u32,
}

impl GpuGeometry {
    pub fn new(geometry: &Geometry, device: &Device) -> Self {
        let mut layouts = Vec::with_capacity(geometry.attributes().len());
        let mut buffers = Vec::new();

        for (kind, attr) in geometry.attributes() {
            log::debug!("attr: {kind:?}");
            layouts.push(OwnedVertexLayout {
                array_stride: attr.format.size(),
                step_mode: VertexStepMode::Vertex,
                attributes: vec![VertexAttribute {
                    format: attr.format,
                    offset: 0,
                    shader_location: kind.shader_location(),
                }],
            });

            buffers.push(device.create_buffer_init(&BufferInitDescriptor {
                label: None,
                contents: attr.data(),
                usage: BufferUsages::COPY_DST | BufferUsages::VERTEX,
            }));
        }

        let index_buffer = geometry.get_index_attribute().map(|attr| {
            device.create_buffer_init(&BufferInitDescriptor {
                label: Some("idx_buf"),
                contents: attr.data(),
                usage: BufferUsages::COPY_DST | BufferUsages::INDEX,
            })
        });

        let mut index_format = None;
        let count = if let Some(idx_attr) = geometry.get_index_attribute() {
            index_format = Some(idx_attr.format);
            idx_attr.count
        } else if let Some(attr) = geometry.get_attribute(AttributeKind::Position) {
            attr.count
        } else if let Some((_, attr)) = geometry.attributes().next() {
            attr.count
        } else {
            0
        };

        Self {
            layout_info: GeneratedVertexLayout { layouts },
            buffers,
            index_buffer,
            index_format,
            count,
        }
    }

    pub fn vertex_layout(&self) -> &GeneratedVertexLayout {
        &self.layout_info
    }

    pub fn vertex_buffers(&self) -> &[Buffer] {
        &self.buffers
    }

    pub fn index_buffer(&self) -> Option<&Buffer> {
        self.index_buffer.as_ref()
    }

    pub fn index_format(&self) -> Option<&IndexFormat> {
        self.index_format.as_ref()
    }

    pub fn count(&self) -> u32 {
        self.count
    }
}

pub struct GeneratedVertexLayout {
    layouts: Vec<OwnedVertexLayout>,
}

impl GeneratedVertexLayout {
    pub fn wgpu_layout(&self) -> Vec<Option<VertexBufferLayout<'_>>> {
        self.layouts.iter().map(|l| Some(l.as_wgpu())).collect()
    }
}

/// Mirror of ['VertexbufferLayout'] that owns the attributes
struct OwnedVertexLayout {
    array_stride: u64,
    step_mode: VertexStepMode,
    attributes: Vec<VertexAttribute>,
}

impl OwnedVertexLayout {
    fn as_wgpu(&self) -> VertexBufferLayout<'_> {
        VertexBufferLayout {
            array_stride: self.array_stride,
            step_mode: self.step_mode,
            attributes: &self.attributes,
        }
    }
}
