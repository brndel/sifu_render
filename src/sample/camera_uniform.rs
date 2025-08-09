use cgmath::{Matrix4, PerspectiveFov, Rad, Transform, Vector2, Vector3};
use sifu_render_derive::Uniform;
use wgpu::Sampler;

use crate::{self as sifu_render, texture::{ColorPixel, ImageTexture}, uniform_binding::UniformBinding, UniformBuffer};

use super::sample_vertex::SampleUniform;

pub struct Camera {
    pub position: Vector3<f32>,
    pub rotation: cgmath::Basis3<f32>,
    pub fovy: Rad<f32>,
    pub screen_size: Vector2<f32>,
}

impl Camera {
    pub fn uniform(&self) -> CameraUniform {
        let transform = cgmath::Decomposed {
            scale: 1.0,
            rot: self.rotation,
            disp: self.position,
        };

        let view: Matrix4<_> = transform.inverse_transform().unwrap().into();

        let aspect = self.screen_size.x / self.screen_size.y;

        let proj = PerspectiveFov {
            fovy: self.fovy,
            aspect,
            near: 0.1,
            far: 1000.0,
        };

        let proj: Matrix4<_> = proj.into();

        CameraUniform {
            view_proj: proj * view,
        }
    }
}

#[derive(Uniform)]
pub struct CameraUniform {
    #[raw(f32; 4; 4)]
    pub view_proj: Matrix4<f32>,
}



#[derive(UniformBinding)]
pub struct FooUniformsDerive<'a> {
    pub sample: &'a UniformBuffer<SampleUniform>,
    pub camera: &'a UniformBuffer<CameraUniform>,
    pub texture: &'a ImageTexture<ColorPixel>,
    pub tex_sampler: &'a Sampler,
}
