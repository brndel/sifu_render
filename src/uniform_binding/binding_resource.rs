use wgpu::{BindingResource, BindingType, Sampler};

use crate::{texture::{ImageTexture, PixelFormat, RenderTexture, TextureRef}, uniform_binding::GlslUniformType, Uniform, UniformBuffer};


pub trait AsBindingResource {
    const LAYOUT: BindingType;
    fn bind_resource(&self) -> BindingResource;
    fn glsl_type() -> GlslUniformType;
}

impl<T: AsBindingResource> AsBindingResource for &T {
    const LAYOUT: BindingType = T::LAYOUT;

    fn bind_resource(&self) -> BindingResource {
        <T as AsBindingResource>::bind_resource(&self)
    }

    fn glsl_type() -> GlslUniformType {
        T::glsl_type()
    }
}

impl<T: Uniform> AsBindingResource for UniformBuffer<T> {
    const LAYOUT: BindingType = BindingType::Buffer {
        ty: wgpu::BufferBindingType::Uniform,
        has_dynamic_offset: false,
        min_binding_size: None,
    };

    fn bind_resource(&self) -> BindingResource {
        BindingResource::Buffer(self.binding())
    }

    fn glsl_type() -> GlslUniformType {
        GlslUniformType {
            is_uniform: true,
            type_name: T::shader_struct_name(),
            type_struct_str: Some(T::shader_struct_str()),
        }
    }
}

impl<P: PixelFormat> AsBindingResource for ImageTexture<P> {
    const LAYOUT: BindingType = BindingType::Texture {
        sample_type: P::TEXTURE_SAMPLE_TYPE,
        view_dimension: wgpu::TextureViewDimension::D2,
        multisampled: false,
    };

    fn bind_resource(&self) -> BindingResource {
        BindingResource::TextureView(self.view())
    }

    fn glsl_type() -> GlslUniformType {
        GlslUniformType {
            is_uniform: false,
            type_name: P::GLSL_TEXTURE_TYPE,
            type_struct_str: None,
        }
    }
}

impl<P: PixelFormat, const MULTISAMPLE: bool> AsBindingResource for RenderTexture<P, MULTISAMPLE> {
    const LAYOUT: BindingType = BindingType::Texture {
        sample_type: P::TEXTURE_SAMPLE_TYPE,
        view_dimension: wgpu::TextureViewDimension::D2,
        multisampled: MULTISAMPLE,
    };

    fn bind_resource(&self) -> BindingResource {
        BindingResource::TextureView(self.view())
    }

    fn glsl_type() -> GlslUniformType {
        GlslUniformType {
            is_uniform: false,
            type_name: P::GLSL_TEXTURE_TYPE,
            type_struct_str: None,
        }
    }
}

impl<'a, P, const MULTISAMPLE: bool> AsBindingResource for TextureRef<'a, P, MULTISAMPLE>
where
    P: PixelFormat,
{
    const LAYOUT: BindingType = BindingType::Texture {
        sample_type: P::TEXTURE_SAMPLE_TYPE,
        view_dimension: wgpu::TextureViewDimension::D2,
        multisampled: MULTISAMPLE,
    };

    fn bind_resource(&self) -> BindingResource {
        BindingResource::TextureView(self.view)
    }

    fn glsl_type() -> GlslUniformType {
        GlslUniformType {
            is_uniform: false,
            type_name: P::GLSL_TEXTURE_TYPE,
            type_struct_str: None,
        }
    }
}

impl AsBindingResource for Sampler {
    const LAYOUT: BindingType = BindingType::Sampler(wgpu::SamplerBindingType::Filtering);

    fn bind_resource(&self) -> BindingResource {
        BindingResource::Sampler(self)
    }

    fn glsl_type() -> GlslUniformType {
        GlslUniformType {
            is_uniform: false,
            type_name: "sampler",
            type_struct_str: None,
        }
    }
}
