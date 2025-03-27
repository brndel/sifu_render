use super::{Vertex, wgpu::VertexBufferLayout};

pub trait MeshInstance: Sized {
    type Vertex: Vertex;

    type Raw: bytemuck::Pod + bytemuck::Zeroable + From<Self>;
    const LAYOUT: VertexBufferLayout<'static>;

    fn shader_struct_str() -> &'static str;
}
