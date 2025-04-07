use wgpu::{BufferUsages, Device};

use crate::Uniform;

use super::{BufferType, GpuBuffer};

pub struct UniformBuf;

impl BufferType for UniformBuf {
    type Extra = ();
}

impl<T: Uniform> GpuBuffer<T, UniformBuf> {
    pub fn uniform(device: &Device, value: T) -> Self {
        let raw: T::Raw = value.into();

        Self::new_raw(device, &[raw], BufferUsages::UNIFORM, ())
    }
}
