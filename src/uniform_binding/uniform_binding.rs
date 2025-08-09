use std::{any::type_name, fmt::Display, sync::OnceLock};

pub use sifu_render_derive::UniformBinding;
use wgpu::{
    BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayout, BindGroupLayoutDescriptor,
    BindGroupLayoutEntry, Device, Sampler, ShaderStages,
};

use crate::{
    gpu_buffer::UniformBuffer,
    sample::{camera_uniform::CameraUniform, sample_vertex::SampleUniform},
    texture::{ColorPixel, ImageTexture},
    uniform_binding::AsBindingResource,
};

pub trait UniformBinding {
    const LAYOUT: &'static [BindGroupLayoutEntry];

    fn binding_entries(&self) -> Vec<BindGroupEntry>;

    fn glsl_vars(group_id: u32) -> Vec<GlslUniformVar>;

    fn bind_group_layout(device: &Device) -> &'static BindGroupLayout;
}

impl UniformBinding for () {
    const LAYOUT: &'static [BindGroupLayoutEntry] = &[];

    fn binding_entries(&self) -> Vec<BindGroupEntry> {
        Vec::new()
    }

    fn glsl_vars(_: u32) -> Vec<GlslUniformVar> {
        Vec::new()
    }

    fn bind_group_layout(device: &Device) -> &'static BindGroupLayout {
        static LAYOUT: OnceLock<BindGroupLayout> = OnceLock::new();

        LAYOUT.get_or_init(|| {
            device.create_bind_group_layout(&BindGroupLayoutDescriptor {
                label: Some(type_name::<Self>()),
                entries: Self::LAYOUT,
            })
        })
    }
}

pub trait UniformBindingExt: UniformBinding {
    fn binding(&self, device: &Device) -> BindGroup {
        let layout = Self::bind_group_layout(device);

        device.create_bind_group(&BindGroupDescriptor {
            label: Some(type_name::<Self>()),
            layout,
            entries: &self.binding_entries(),
        })
    }
}

impl<T: UniformBinding> UniformBindingExt for T {}

pub struct FooUniforms<'a> {
    pub sample_uniform: &'a UniformBuffer<SampleUniform>,
    pub camera: &'a UniformBuffer<CameraUniform>,
    pub texture: &'a ImageTexture<ColorPixel>,
    pub sampler: &'a Sampler,
}

impl<'a> UniformBinding for FooUniforms<'a> {
    const LAYOUT: &'static [BindGroupLayoutEntry] = &[
        BindGroupLayoutEntry {
            binding: 0,
            visibility: ShaderStages::VERTEX_FRAGMENT,
            ty: <&'a UniformBuffer<SampleUniform> as AsBindingResource>::LAYOUT,
            count: None,
        },
        BindGroupLayoutEntry {
            binding: 1,
            visibility: ShaderStages::VERTEX_FRAGMENT,
            ty: <&'a ImageTexture<ColorPixel> as AsBindingResource>::LAYOUT,
            count: None,
        },
        BindGroupLayoutEntry {
            binding: 2,
            visibility: ShaderStages::VERTEX_FRAGMENT,
            ty: <&'a Sampler as AsBindingResource>::LAYOUT,
            count: None,
        },
        BindGroupLayoutEntry {
            binding: 3,
            visibility: ShaderStages::VERTEX_FRAGMENT,
            ty: <&'a UniformBuffer<CameraUniform> as AsBindingResource>::LAYOUT,
            count: None,
        },
    ];

    fn binding_entries(&self) -> Vec<BindGroupEntry> {
        vec![
            BindGroupEntry {
                binding: 0,
                resource: self.sample_uniform.bind_resource(),
            },
            BindGroupEntry {
                binding: 1,
                resource: self.texture.bind_resource(),
            },
            BindGroupEntry {
                binding: 2,
                resource: self.sampler.bind_resource(),
            },
            BindGroupEntry {
                binding: 3,
                resource: self.camera.bind_resource(),
            },
        ]
    }

    fn glsl_vars(group_id: u32) -> Vec<GlslUniformVar> {
        vec![GlslUniformVar {
            group_id,
            binding_id: 0,
            name: "sample_uniform",
            uniform: <&'a UniformBuffer<SampleUniform> as AsBindingResource>::glsl_type(),
        }]
    }

    fn bind_group_layout(device: &Device) -> &'static BindGroupLayout {
        static LAYOUT: OnceLock<BindGroupLayout> = OnceLock::new();

        LAYOUT.get_or_init(|| {
            device.create_bind_group_layout(&BindGroupLayoutDescriptor {
                label: Some(type_name::<Self>()),
                entries: Self::LAYOUT,
            })
        })
    }
}

pub struct GlslUniformVar {
    pub group_id: u32,
    pub binding_id: u32,
    pub name: &'static str,
    pub uniform: GlslUniformType,
}

pub struct GlslUniformType {
    pub is_uniform: bool,
    pub type_name: &'static str,
    pub type_struct_str: Option<&'static str>,
}

impl Display for GlslUniformVar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "@group({}) @binding({}) var{} {}: {}",
            self.group_id,
            self.binding_id,
            if self.uniform.is_uniform {
                "<uniform>"
            } else {
                ""
            },
            self.name,
            self.uniform.type_name
        )
    }
}
