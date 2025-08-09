use std::marker::PhantomData;

use wgpu::{TextureFormat, TextureView};

use super::ColorPixel;

pub struct TextureRef<'a, P = ColorPixel, const MULTISAMPLE: bool = false> {
    pub view: &'a TextureView,
    pub format: TextureFormat,
    pub _phantom: PhantomData<P>,
}
