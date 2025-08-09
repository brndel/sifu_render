use wgpu::{Device, RenderPass};

use crate::gpu_buffer::{GpuBuffer, IndexBuf, InstanceBuf, VertexBuf};

use super::{MeshInstance, vertex::Vertex};

pub struct Mesh<V, const PRIMITIVE_SIZE: usize = 3> {
    vertices: GpuBuffer<V, VertexBuf>,
    indices: GpuBuffer<u32, IndexBuf<PRIMITIVE_SIZE>>,
}

impl<V: Vertex, const C: usize> Mesh<V, C> {
    pub fn new(device: &Device, vertices: Vec<V>, indices: Vec<[u32; C]>) -> Self {
        let vertices = GpuBuffer::vertices(device, vertices);
        let indices = GpuBuffer::<u32, _>::indices(device, indices);

        Self { vertices, indices }
    }
}

impl<V, const C: usize> Mesh<V, C> {
    pub fn draw<'a, I>(
        &'a self,
        instances: &'a GpuBuffer<I, InstanceBuf>,
        pass: &mut RenderPass<'a>,
    ) where
        I: MeshInstance<Vertex = V>,
    {
        pass.set_index_buffer(self.indices.slice(), self.indices.index_format());

        // Vertices
        pass.set_vertex_buffer(0, self.vertices.slice());
        // Instance
        pass.set_vertex_buffer(1, instances.slice());
        pass.draw_indexed(0..self.indices.count(), 0, 0..instances.count());
    }
}
