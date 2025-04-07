use cgmath::{Matrix4, Vector3, Vector4};
use wgpu::{
    BufferAddress, Device, VertexAttribute, VertexBufferLayout, VertexFormat, VertexStepMode,
};

use crate::mesh::{Mesh, MeshInstance, Vertex};

use crate::Uniform;
use crate::{self as sifu_render};

pub struct SampleManualVertex {
    position: Vector3<f32>,
    color: Vector4<f32>,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Zeroable, bytemuck::Pod)]
pub struct SampleManualRawVertex {
    position: [f32; 3],
    color: [f32; 4],
}

impl Vertex for SampleManualVertex {
    type Raw = SampleManualRawVertex;
    const LAYOUT: VertexBufferLayout<'static> = VertexBufferLayout {
        array_stride: core::mem::size_of::<SampleManualRawVertex>() as BufferAddress,
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

impl From<SampleManualVertex> for SampleManualRawVertex {
    fn from(value: SampleManualVertex) -> Self {
        Self {
            position: value.position.into(),
            color: value.color.into(),
        }
    }
}

#[derive(Vertex)]
pub struct SampleVertex {
    #[raw(f32; 3)]
    pub position: Vector3<f32>,
    #[raw(f32; 3)]
    pub color: Vector3<f32>,
}

#[derive(MeshInstance)]
#[vertex(SampleVertex)]
pub struct SampleInstance {
    #[raw(f32; 4; 4)]
    pub mat: Matrix4<f32>,
}

#[derive(Uniform)]
pub struct SampleUniform {
    pub value: f32,
    #[raw(f32; 3)]
    pub color: Vector3<f32>,
    // #[raw(u32; 2)]
    // pub thing: Vector2<u32>,
    // #[raw(f32; 2; 2)]
    // pub matrix: Matrix2<f32>,
}

impl SampleVertex {
    pub fn sample_mesh(device: &Device) -> Mesh<Self> {
        let vertices = vec![
            Self {
                position: Vector3 {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
                color: Vector3 {
                    x: 1.0,
                    y: 0.0,
                    z: 0.0,
                },
            },
            Self {
                position: Vector3 {
                    x: 1.0,
                    y: 0.0,
                    z: 0.0,
                },
                color: Vector3 {
                    x: 0.0,
                    y: 1.0,
                    z: 0.0,
                },
            },
            Self {
                position: Vector3 {
                    x: 0.0,
                    y: 1.0,
                    z: 0.0,
                },
                color: Vector3 {
                    x: 0.0,
                    y: 0.0,
                    z: 1.0,
                },
            },
        ];

        let indices = vec![[0, 1, 2]];

        Mesh::new(device, vertices, indices)
    }
}
