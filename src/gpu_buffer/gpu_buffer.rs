use std::marker::PhantomData;

use bytemuck::{Pod, Zeroable, cast_slice};
use wgpu::{
    Buffer, BufferBinding, BufferSlice, BufferUsages, Device,
    util::{BufferInitDescriptor, DeviceExt},
};

pub struct GpuBuffer<T, B: BufferType> {
    buffer: Buffer,
    pub(super) extra: B::Extra,
    _phantom: PhantomData<(T, B)>,
}

impl<T, B: BufferType> GpuBuffer<T, B> {
    pub(super) fn new_raw<D: Pod + Zeroable>(
        device: &Device,
        data: &[D],
        usage: BufferUsages,
        extra: B::Extra,
    ) -> Self {
        let raw_bytes = cast_slice(data);

        let buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: None,
            contents: raw_bytes,
            usage,
        });

        Self {
            buffer,
            extra,
            _phantom: PhantomData,
        }
    }
}

impl<T, B: BufferType> GpuBuffer<T, B> {
    pub fn binding(&self) -> BufferBinding {
        self.buffer.as_entire_buffer_binding()
    }

    pub fn slice(&self) -> BufferSlice {
        self.buffer.slice(..)
    }
}

pub trait BufferType {
    type Extra;
}
