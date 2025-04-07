use std::fmt::Write;

use crate::mesh::{MeshInstance, Vertex};

pub fn preprocess_shader<V: Vertex, I: MeshInstance>(source: &str) -> String {
    let vertex_struct_code = V::shader_struct_str();
    let instance_struct_code = I::shader_struct_str();

    let mut out =
        String::with_capacity(source.len() + vertex_struct_code.len() + instance_struct_code.len());

    out += vertex_struct_code;
    out += "\n";

    out += instance_struct_code;
    out += "\n";

    out += source;
    
    out
}
