pub use sifu_render_derive::Uniform;

pub trait Uniform: Sized {
    type Raw: bytemuck::Pod + bytemuck::Zeroable + From<Self>;

    fn shader_struct_str() -> &'static str;
    fn shader_struct_name() -> &'static str;
}