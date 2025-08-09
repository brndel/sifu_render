mod gpu_buffer;
mod index;
mod instance;
mod uniform;
mod vertex;

pub use gpu_buffer::BufferType;
pub use gpu_buffer::GpuBuffer;

pub use index::IndexBuf;
pub use instance::InstanceBuf;
pub use uniform::UniformBuf;
pub use uniform::UniformExt;
pub use vertex::VertexBuf;

pub type UniformBuffer<T> = GpuBuffer<T, UniformBuf>;
pub type VertexBuffer<T> = GpuBuffer<T, VertexBuf>;
pub type IndexBuffer<T, const PRIMITIVE_SIZE: usize> = GpuBuffer<T, IndexBuf<PRIMITIVE_SIZE>>;
pub type InstanceBuffer<T> = GpuBuffer<T, InstanceBuf>;
