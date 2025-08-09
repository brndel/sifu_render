use std::marker::PhantomData;

use cgmath::Vector2;
use wgpu::{
    Device, Extent3d, SurfaceTexture, Texture, TextureDescriptor, TextureFormat, TextureUsages,
    TextureView, TextureViewDescriptor,
};

use super::{PixelFormat, TextureRef};

pub struct RenderTexture<P, const MULTISAMPLE: bool = false> {
    size: Vector2<u32>,
    texture: TextureStore,
    view: TextureView,

    format: TextureFormat,
    _phantom: PhantomData<P>,
}

#[allow(unused)]
enum TextureStore {
    OnlyTextureView,
    Texture(Texture),
    SurfaceTexture(SurfaceTexture),
}

impl<P: PixelFormat, const MULTISAMPLE: bool> RenderTexture<P, MULTISAMPLE> {
    pub fn new(device: &Device, size: Vector2<u32>) -> Self {
        Self::new_any_format(device, size, P::FORMAT)
    }
}

impl<const MULTISAMPLE: bool> RenderTexture<(), MULTISAMPLE> {
    pub fn new_format(device: &Device, size: Vector2<u32>, format: TextureFormat) -> Self {
        Self::new_any_format(device, size, format)
    }
}

impl<P, const MULTISAMPLE: bool> RenderTexture<P, MULTISAMPLE> {
    fn new_any_format(device: &Device, size: Vector2<u32>, format: TextureFormat) -> Self {
        let texture = device.create_texture(&TextureDescriptor {
            label: None,
            size: Extent3d {
                width: size.x,
                height: size.y,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: Self::sample_count(),
            dimension: wgpu::TextureDimension::D2,
            format,
            usage: Self::TEXTURE_USAGES,
            view_formats: &[],
        });

        let view = texture.create_view(&TextureViewDescriptor::default());

        Self {
            size,
            texture: TextureStore::Texture(texture),
            view,
            format,
            _phantom: PhantomData,
        }
    }

    pub fn resize(&mut self, device: &Device, size: Vector2<u32>) {
        if self.size != size {
            *self = Self::new_any_format(device, size, self.format);
        }
    }

    pub fn to_any_format(self) -> RenderTexture<(), MULTISAMPLE> {
        RenderTexture {
            size: self.size,
            texture: self.texture,
            view: self.view,
            format: self.format,
            _phantom: PhantomData,
        }
    }

    pub fn format(&self) -> TextureFormat {
        self.format
    }

    fn sample_count() -> u32 {
        if MULTISAMPLE { 4 } else { 1 }
    }
}

impl RenderTexture<(), false> {
    pub fn from_view(view: TextureView, size: Vector2<u32>, format: TextureFormat) -> Self {
        Self {
            size,
            texture: TextureStore::OnlyTextureView,
            view,
            format,
            _phantom: PhantomData,
        }
    }

    pub fn from_surface_texture(surface_texture: SurfaceTexture) -> Self {
        let format = surface_texture.texture.format();

        let view = surface_texture
            .texture
            .create_view(&TextureViewDescriptor::default());

        let size = {
            let size = surface_texture.texture.size();
            Vector2::new(size.width, size.height)
        };

        Self {
            size,
            texture: TextureStore::SurfaceTexture(surface_texture),
            view,
            format,
            _phantom: PhantomData,
        }
    }

    pub fn present(self) {
        match self.texture {
            TextureStore::SurfaceTexture(surface_texture) => surface_texture.present(),
            _ => (),
        }
    }
}

impl<P, const MULTISAMPLE: bool> RenderTexture<P, MULTISAMPLE> {
    const TEXTURE_USAGES: TextureUsages =
        TextureUsages::TEXTURE_BINDING.union(TextureUsages::RENDER_ATTACHMENT);

    pub fn view(&self) -> &TextureView {
        &self.view
    }

    pub fn size(&self) -> Vector2<u32> {
        self.size
    }

    pub fn texture_ref(&self) -> TextureRef<P, MULTISAMPLE> {
        TextureRef {
            view: self.view(),
            format: self.format(),
            _phantom: PhantomData,
        }
    }
}
