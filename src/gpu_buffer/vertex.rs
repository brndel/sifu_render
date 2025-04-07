use wgpu::{BufferUsages, Device};

use crate::mesh::Vertex;

use super::{BufferType, GpuBuffer};


pub struct VertexBuf;

impl BufferType for VertexBuf {
    type Extra = u32;
}

impl<T: Vertex> GpuBuffer<T, VertexBuf> {
    pub fn vertices(device: &Device, vertices: impl IntoIterator<Item = T>) -> Self {
        let raw: Vec<T::Raw> = vertices.into_iter().map(Into::into).collect();

        Self::new_raw(device, &raw, BufferUsages::VERTEX, raw.len() as u32)
    }
}
