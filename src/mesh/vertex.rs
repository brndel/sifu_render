use super::wgpu::VertexBufferLayout;

pub trait Vertex: Sized {
    type Raw: bytemuck::Pod + bytemuck::Zeroable + From<Self>;

    const LAYOUT: VertexBufferLayout<'static>;
    const SHADER_LOCATION_COUNT: u32;

    fn shader_struct_str() -> &'static str;
}
