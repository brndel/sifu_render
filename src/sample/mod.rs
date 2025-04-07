use sample_vertex::{SampleInstance, SampleVertex};

use crate::shader::preprocess_shader;

pub mod sample_vertex;


pub fn sample_shader() -> String {
    preprocess_shader::<SampleVertex, SampleInstance>(include_str!("sample_shader.wgsl"))
}