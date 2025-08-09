use std::marker::PhantomData;

use bytemuck::cast_slice;
use cgmath::Vector2;
use wgpu::{
    util::DeviceExt, Device, Extent3d, Queue, TexelCopyBufferLayout, Texture, TextureDescriptor, TextureUsages, TextureView, TextureViewDescriptor
};

use super::{ColorPixel, PixelFormat, TextureRef};
use std::path::Path;

pub struct ImageTexture<P = ColorPixel> {
    size: Vector2<u32>,
    texture: Texture,
    view: TextureView,

    _phantom: PhantomData<P>,
}

impl<P: PixelFormat> ImageTexture<P> {
    pub fn new(device: &Device, queue: &Queue, size: Vector2<u32>, data: &[P::Pixel]) -> Self {
        assert!(data.len() == (size.x * size.y) as usize);

        let data = cast_slice(data);

        let texture = device.create_texture_with_data(
            queue,
            &TextureDescriptor {
                label: None,
                size: Extent3d {
                    width: size.x,
                    height: size.y,
                    depth_or_array_layers: 1,
                },
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: P::FORMAT,
                usage: Self::TEXTURE_USAGES,
                view_formats: &[],
            },
            wgpu::util::TextureDataOrder::LayerMajor,
            data,
        );

        let view = texture.create_view(&TextureViewDescriptor::default());

        Self {
            size,
            texture,
            view,
            _phantom: PhantomData,
        }
    }

    pub fn new_procedural(
        device: &Device,
        queue: &Queue,
        size: Vector2<u32>,
        f: impl Fn(Vector2<u32>) -> P::Pixel,
    ) -> Self {
        let indices = (0..size.y).flat_map(|y| (0..size.x).map(move |x| Vector2::new(x, y)));

        let data = indices.map(f).collect::<Vec<_>>();

        Self::new(device, queue, size, &data)
    }

    pub fn write(&self, queue: &Queue, data: &[P::Pixel]) {
        assert!(data.len() == (self.size.x * self.size.y) as usize);

        let data = cast_slice(data);

        queue.write_texture(
            self.texture.as_image_copy(),
            data,
            TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(self.size.x * size_of::<P::Pixel>() as u32),
                rows_per_image: Some(self.size.y),
            },
            Extent3d {
                width: self.size.x,
                height: self.size.y,
                depth_or_array_layers: 1,
            },
        );
    }
}

#[cfg(feature = "image")]
impl ImageTexture<ColorPixel> {
    pub fn load_image(
        device: &Device,
        queue: &Queue,
        path: impl AsRef<Path>,
    ) -> image::ImageResult<Self> {
        let img = image::open(path)?;

        Self::from_dynamic_img(device, queue, img)
    }

    #[cfg(feature = "image")]
    pub fn load_image_from_memory(
        device: &Device,
        queue: &Queue,
        data: &[u8],
    ) -> image::ImageResult<Self> {
        let img = image::load_from_memory(data)?;

        Self::from_dynamic_img(device, queue, img)
    }

    fn from_dynamic_img(
        device: &Device,
        queue: &Queue,
        img: image::DynamicImage,
    ) -> image::ImageResult<Self> {
        use image::GenericImageView;

        let (width, height) = img.dimensions();

        let data_raw = img.to_rgba8().to_vec();

        let data = cast_slice(&data_raw);

        Ok(Self::new(device, queue, Vector2::new(width, height), &data))
    }
}

impl<P> ImageTexture<P> {
    const TEXTURE_USAGES: TextureUsages = TextureUsages::TEXTURE_BINDING;

    pub fn view(&self) -> &TextureView {
        &self.view
    }

    pub fn size(&self) -> Vector2<u32> {
        self.size
    }

    pub fn texture_ref(&self) -> TextureRef<P, false>
    where
        P: PixelFormat,
    {
        TextureRef {
            view: self.view(),
            format: P::FORMAT,
            _phantom: PhantomData,
        }
    }
}
