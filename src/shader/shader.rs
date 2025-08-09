use std::marker::PhantomData;

use wgpu::{Device, PipelineCompilationOptions, ShaderModule, ShaderModuleDescriptor, VertexState};

use crate::{
    mesh::{MeshInstance, Vertex},
    uniform_binding::UniformBinding,
};

pub struct Shader<V, I, U0 = (), U1 = ()> {
    module: ShaderModule,
    code: String,

    _phantom: PhantomData<(V, I, U0, U1)>,
}

impl<V, I, U0, U1> Shader<V, I, U0, U1> {
    pub const ENTRY_POINT_VERTEX: &'static str = "vertex";
    pub const ENTRY_POINT_FRAGMENT: &'static str = "fragment";
}

impl<V: Vertex, I: MeshInstance, U0: UniformBinding, U1: UniformBinding> Shader<V, I, U0, U1> {
    pub fn new(device: &Device, source: &str) -> Self {
        let code = Self::preprocess_shader(source);

        let module = device.create_shader_module(ShaderModuleDescriptor {
            label: Some("Shader"),
            source: wgpu::ShaderSource::Wgsl(code.as_str().into()),
        });

        Self {
            module,
            code,
            _phantom: PhantomData,
        }
    }

    pub fn code(&self) -> &str {
        &self.code
    }

    fn preprocess_shader(source: &str) -> String {
        let vertex_struct_code = V::shader_struct_str();
        let instance_struct_code = I::shader_struct_str();

        let mut out = String::with_capacity(
            source.len() + vertex_struct_code.len() + instance_struct_code.len(),
        );

        out += vertex_struct_code;
        out += "\n";

        out += instance_struct_code;
        out += "\n";

        Self::append_uniforms::<U0>(0, &mut out);
        Self::append_uniforms::<U1>(1, &mut out);

        out += source;

        out
    }

    fn append_uniforms<T: UniformBinding>(group_id: u32, out: &mut String) {
        let glsl_vars = T::glsl_vars(group_id);
        for struct_str in glsl_vars
            .iter()
            .filter_map(|var| var.uniform.type_struct_str)
        {
            *out += struct_str;
            *out += "\n";
        }

        for var in &glsl_vars {
            *out += &var.to_string();
            *out += ";\n";
        }
    }
}

impl<V: Vertex, I: MeshInstance, U0, U1> Shader<V, I, U0, U1> {
    pub fn module(&self) -> &ShaderModule {
        &self.module
    }

    pub fn vertex_state(&self) -> VertexState {
        VertexState {
            module: &self.module,
            entry_point: Some(Self::ENTRY_POINT_VERTEX),
            buffers: &[V::LAYOUT, I::LAYOUT],
            compilation_options: PipelineCompilationOptions::default(),
        }
    }
}
