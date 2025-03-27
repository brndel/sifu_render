use wgpu::Device;

use crate::gpu_buffer::GpuBuffer;

use super::vertex::Vertex;

pub struct Mesh<V> {
    vertices: GpuBuffer<V>,
    indices: GpuBuffer<u32>,
}

impl<V: Vertex> Mesh<V> {
    // pub fn new(device: &Device, vertices: Vec<V>, indices: Vec<u32>) -> Self {
    //     let vertices = GpuBuffer::new_vec(device, vertices);
    //     let indices = GpuBuffer::new_vec(device, indices);

    //     Self { vertices, indices }
    // }
}
