use camera_uniform::FooUniformsDerive;
use sample_vertex::{SampleInstance, SampleVertex};
use wgpu::Device;

use crate::shader::Shader;

pub mod camera_uniform;
pub mod sample_vertex;

pub fn sample_shader(device: &Device) -> Shader<SampleVertex, SampleInstance, FooUniformsDerive> {
    Shader::new(device, include_str!("sample_shader.wgsl"))
}
