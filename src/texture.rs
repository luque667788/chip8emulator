use anyhow::*;
use image::{DynamicImage, GenericImageView};

pub struct Texture {
    #[allow(unused)]
    pub texture: wgpu::Texture,
    pub view: wgpu::TextureView,
    pub sampler: wgpu::Sampler,
}

impl Texture {

    

    
    pub fn from_bytes(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        bytes: &[u8],
        label: &str,
    ) -> Result<Self> {
        let img = image::load_from_memory(bytes)?;
        Self::from_image(device, queue, &img, Some(label))
    }

    pub fn from_image(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        img: &image::DynamicImage,
        label: Option<&str>,
    ) -> Result<Self> {
        let rgba = img.to_rgba8();
        let dimensions = img.dimensions();

        let size = wgpu::Extent3d {
            width: dimensions.0,
            height: dimensions.1,
            depth_or_array_layers: 1,
        };
        let format = wgpu::TextureFormat::Rgba8UnormSrgb;
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label,
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });

        queue.write_texture(
            wgpu::ImageCopyTexture {
                aspect: wgpu::TextureAspect::All,
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
            },
            &rgba,
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(4 * dimensions.0),
                rows_per_image: Some(dimensions.1),
            },
            size,
        );

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });

        Ok(Self {
            texture,
            view,
            sampler,
        })
    }
}

pub fn create_img(size: winit::dpi::PhysicalSize<u32>,sizex: u32,sizey: u32) -> DynamicImage {

    let sizex = 6;
    let sizey = 3;

    #[rustfmt::skip]
    let screen = [
        false, false, true,
        false, false, false,
        false, true, false,
        false, false, true,
        false, false, false,
        false, true, false,
    ];


    


    let img = bool_array_to_image_buffer(&screen, sizex as u32, sizey as u32);
    let resized_img = resize_image_buffer(&img, sizex*100, sizey*100);
    // Convert the `ImageBuffer` to a `DynamicImage`
    DynamicImage::ImageRgba8(resized_img)

}

pub fn bool_array_to_image_buffer(bool_array: &[bool], width: u32, height: u32) -> image::RgbaImage {
    let mut img = image::RgbaImage::new(width, height);
    for (i, &pixel) in bool_array.iter().enumerate() {
        let x = (i as u32) % width;
        let y = (i as u32) / width;
        let color = if pixel {
            image::Rgba([255, 255, 255, 255]) // White
        } else {
            image::Rgba([0, 0, 0, 255]) // Black
        };
        img.put_pixel(x, y, color);
    }
    img
}

pub fn resize_image_buffer(img: &image::RgbaImage, new_width: u32, new_height: u32) -> image::RgbaImage {
    image::imageops::resize(img, new_width, new_height, image::imageops::FilterType::Nearest)
}