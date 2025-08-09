use bytemuck::cast_slice;
use wgpu::{BufferUsages, Device, Queue};

use crate::Uniform;

use super::{BufferType, GpuBuffer};

pub struct UniformBuf;

impl BufferType for UniformBuf {
    type Extra = ();
}

impl<T: Uniform> GpuBuffer<T, UniformBuf> {
    pub fn uniform(device: &Device, value: T) -> Self {
        let raw: T::Raw = value.into();

        Self::new_raw(
            device,
            &[raw],
            BufferUsages::UNIFORM | BufferUsages::COPY_DST,
            (),
        )
    }

    pub fn update(&mut self, queue: &Queue, value: T) {
        let raw: T::Raw = value.into();

        queue.write_buffer(&self.buffer, 0, cast_slice(&[raw]));
    }
}

pub trait UniformExt: Sized {
    fn buffer(self, device: &Device) -> GpuBuffer<Self, UniformBuf>;
}

impl<T: Uniform> UniformExt for T {
    fn buffer(self, device: &Device) -> GpuBuffer<Self, UniformBuf> {
        GpuBuffer::uniform(device, self)
    }
}
