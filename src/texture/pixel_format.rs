use bytemuck::{Pod, Zeroable};
use wgpu::{TextureFormat, TextureSampleType};

use super::Color;

pub trait PixelFormat {
    type Pixel: Pod + Zeroable + Sized;
    const FORMAT: TextureFormat;
    const TEXTURE_SAMPLE_TYPE: TextureSampleType;
    const GLSL_TEXTURE_TYPE: &'static str;
}

pub struct DepthStencilPixel;

impl PixelFormat for DepthStencilPixel {
    type Pixel = ();
    const FORMAT: TextureFormat = TextureFormat::Depth24PlusStencil8;

    const TEXTURE_SAMPLE_TYPE: TextureSampleType = TextureSampleType::Depth;
    const GLSL_TEXTURE_TYPE: &'static str = "texture_depth_2d";
}

pub struct ColorPixel;

impl PixelFormat for ColorPixel {
    type Pixel = Color;
    const FORMAT: TextureFormat = TextureFormat::Rgba8UnormSrgb;

    const TEXTURE_SAMPLE_TYPE: TextureSampleType = TextureSampleType::Float { filterable: true };
    const GLSL_TEXTURE_TYPE: &'static str = "texture_2d<f32>";
}

pub struct DisplayPixel;

impl PixelFormat for DisplayPixel {
    type Pixel = ();
    const FORMAT: TextureFormat = TextureFormat::Bgra8UnormSrgb;
    const TEXTURE_SAMPLE_TYPE: TextureSampleType = TextureSampleType::Float { filterable: true };
    const GLSL_TEXTURE_TYPE: &'static str = "texture_2d<f32>";
}
