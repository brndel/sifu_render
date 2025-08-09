use bytemuck::cast_slice;
use wgpu::{BufferUsages, Device, Queue};

use crate::mesh::MeshInstance;

use super::{BufferType, GpuBuffer};

pub struct InstanceBuf;

impl BufferType for InstanceBuf {
    type Extra = InstanceCount;
}

pub struct InstanceCount {
    count: u32,
    capacity: u32,
}

impl InstanceCount {
    fn new(count: u32) -> Self {
        Self {
            count,
            capacity: count,
        }
    }
}

impl<T> GpuBuffer<T, InstanceBuf> {
    pub fn count(&self) -> u32 {
        self.extra.count
    }
}

impl<T: MeshInstance> GpuBuffer<T, InstanceBuf> {
    pub fn instances(device: &Device, instances: impl IntoIterator<Item = T>) -> Self {
        let raw: Vec<T::Raw> = instances.into_iter().map(Into::into).collect();

        Self::new_raw(
            device,
            &raw,
            BufferUsages::VERTEX | BufferUsages::COPY_DST,
            InstanceCount::new(raw.len() as u32),
        )
    }

    pub fn update(
        &mut self,
        device: &Device,
        queue: &Queue,
        instances: impl IntoIterator<Item = T>,
    ) {
        let raw: Vec<T::Raw> = instances.into_iter().map(Into::into).collect();

        if raw.len() > self.extra.capacity as usize {
            queue.write_buffer(&self.buffer, 0, cast_slice(&raw));
            self.extra.count = raw.len() as u32;
        } else {
            *self = Self::new_raw(
                device,
                &raw,
                BufferUsages::VERTEX | BufferUsages::COPY_DST,
                InstanceCount::new(raw.len() as u32),
            );
        }
    }
}
