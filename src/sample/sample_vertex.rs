use bytemuck::{Pod, Zeroable};
use cgmath::{Matrix4, Vector2, Vector3, Vector4};
use wgpu::{
    BufferAddress, BufferSize, VertexAttribute, VertexBufferLayout, VertexFormat, VertexStepMode,
    vertex_attr_array,
};

use crate::mesh::{MeshInstance, Vertex};

use crate::Uniform;
use crate::{self as sifu_render};

pub struct SampleVertex {
    position: Vector3<f32>,
    color: Vector4<f32>,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Zeroable, bytemuck::Pod)]
pub struct SampleRawVertex {
    position: [f32; 3],
    color: [f32; 4],
}

impl Vertex for SampleVertex {
    type Raw = SampleRawVertex;
    const LAYOUT: VertexBufferLayout<'static> = VertexBufferLayout {
        array_stride: core::mem::size_of::<SampleRawVertex>() as BufferAddress,
        step_mode: VertexStepMode::Vertex,
        attributes: &[
            VertexAttribute {
                format: VertexFormat::Float32x3,
                offset: 0,
                shader_location: 0,
            },
            VertexAttribute {
                format: VertexFormat::Float32x3,
                offset: 12,
                shader_location: 1,
            },
        ],
    };

    const SHADER_LOCATION_COUNT: u32 = 2;

    fn shader_struct_str() -> &'static str {
        "struct SampleVertex {
            @location(0) position: vec3<f32>,
            @location(1) color: vec3<f32>,
        }"
    }
}

impl From<SampleVertex> for SampleRawVertex {
    fn from(value: SampleVertex) -> Self {
        Self {
            position: value.position.into(),
            color: value.color.into(),
        }
    }
}


#[derive(Vertex)]
pub struct SampleDeriveVertex {
    #[raw(f32; 3)]
    position: Vector3<f32>,
    #[raw(f32; 4)]
    color: Vector4<f32>,
    #[raw(f32; 4; 4)]
    mat: Matrix4<f32>,
}


#[derive(MeshInstance)]
#[vertex(SampleDeriveVertex)]
pub struct SampleDeriveInstance {
    #[raw(f32; 4; 4)]
    mat: Matrix4<f32>
}


#[derive(Uniform)]
pub struct SampleDeriveUniform {
    #[raw(f32; 3)]
    color: Vector3<f32>,
    #[raw(u32; 2)]
    thing: Vector2<u32>
}