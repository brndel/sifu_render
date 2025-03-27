use std::marker::PhantomData;

use bytemuck::cast_slice;
use wgpu::{util::{BufferInitDescriptor, DeviceExt}, Buffer, BufferUsages, Device};



pub struct GpuBuffer<T> {
    buffer: Buffer,
    _phantom: PhantomData<T>
}

impl<T: ToBufferValue> GpuBuffer<T> {
    pub fn new(device: &Device, data: T) -> Self {
        let raw = T::Raw::from(data);

        let raw_arr = [raw];
        let raw_bytes = cast_slice(&raw_arr);

        let buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: None,
            contents: raw_bytes,
            usage: BufferUsages::VERTEX | BufferUsages::COPY_DST,
        });

        Self { buffer, _phantom: PhantomData }
    }
    pub fn new_vec(device: &Device, data: Vec<T>) -> Self {
        let raw = data.into_iter().map(T::Raw::from).collect::<Vec<_>>();

        let raw_bytes = cast_slice(&raw);


        let buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: None,
            contents: raw_bytes,
            usage: BufferUsages::VERTEX | BufferUsages::COPY_DST,
        });

        Self { buffer, _phantom: PhantomData }
    }
}

pub trait ToBufferValue: Sized {
    type Raw: bytemuck::Pod + bytemuck::Zeroable + From<Self>;
}

impl<T: bytemuck::Pod + bytemuck::Zeroable> ToBufferValue for T {
    type Raw = Self;
}