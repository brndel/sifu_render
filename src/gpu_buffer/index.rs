use std::fmt::Debug;

use wgpu::{BufferUsages, Device, IndexFormat};

use super::{BufferType, GpuBuffer};

pub struct IndexBuf<const PRIMITIVE_SIZE: usize>;
impl<const C: usize> BufferType for IndexBuf<C> {
    type Extra = (u32, wgpu::IndexFormat);
}

impl<T: TryInto<u16> + Into<u32> + Copy, const C: usize> GpuBuffer<T, IndexBuf<C>>
where
    T::Error: Debug,
{
    /// Can be used with `u16` and `u32` indices.
    ///
    /// If the all indices can be stored as a `u16`, `indices` gets automaticly converted and the indices are stored as `u16`. If not, they are stored as `u32`
    pub fn indices(device: &Device, indices: Vec<[T; C]>) -> Self {
        let needs_u32 = indices
            .iter()
            .flatten()
            .any(|i| TryInto::<u16>::try_into(*i).is_err());

        if needs_u32 {
            let indices = indices
                .into_iter()
                .flatten()
                .map(Into::<u32>::into)
                .collect::<Vec<_>>();
            Self::new_raw(
                device,
                &indices,
                BufferUsages::INDEX,
                (indices.len() as u32, IndexFormat::Uint32),
            )
        } else {
            let indices = indices
                .into_iter()
                .flatten()
                .map(TryInto::<u16>::try_into)
                .map(Result::unwrap)
                .collect::<Vec<_>>();
            Self::new_raw(
                device,
                &indices,
                BufferUsages::INDEX,
                (indices.len() as u32, IndexFormat::Uint16),
            )
        }
    }
}

impl<T, const C: usize> GpuBuffer<T, IndexBuf<C>> {
    pub fn count(&self) -> u32 {
        self.extra.0
    }

    pub fn index_format(&self) -> IndexFormat {
        self.extra.1
    }
}
