use wgpu::{BufferUsages, Device};

use crate::mesh::MeshInstance;

use super::{BufferType, GpuBuffer};

pub struct InstanceBuf;

impl BufferType for InstanceBuf {
    type Extra = u32;
}

impl<T> GpuBuffer<T, InstanceBuf> {
    pub fn count(&self) -> u32 {
        self.extra
    }
}

impl<T: MeshInstance> GpuBuffer<T, InstanceBuf> {
    pub fn instances(device: &Device, instances: impl IntoIterator<Item = T>) -> Self {
        let raw: Vec<T::Raw> = instances.into_iter().map(Into::into).collect();

        Self::new_raw(device, &raw, BufferUsages::VERTEX, raw.len() as u32)
    }
}
