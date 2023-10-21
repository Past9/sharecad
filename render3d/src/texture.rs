use image::GenericImageView;
use std::{cell::OnceCell, sync::Arc};

pub enum TextureImage {
    Depth,
    Diffuse(image::DynamicImage),
    NormalMap(image::DynamicImage),
}
impl std::fmt::Debug for TextureImage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Depth => write!(f, "Depth"),
            Self::Diffuse(image) => {
                let dims = image.dimensions();
                f.write_fmt(format_args!("Diffuse(<{}x{} image>)", dims.0, dims.1))
            }
            Self::NormalMap(image) => {
                let dims = image.dimensions();
                f.write_fmt(format_args!("NormalMap(<{}x{} image>)", dims.0, dims.1))
            }
        }
    }
}

pub enum ImageTextureKind {
    Diffuse,
    NormalMap,
}

#[derive(Debug)]
pub struct TextureResources {
    pub texture: Arc<wgpu::Texture>,
    pub view: Arc<wgpu::TextureView>,
    pub sampler: Arc<wgpu::Sampler>,
}

#[derive(Debug)]
pub struct Texture {
    pub image: TextureImage,
    pub label: String,
    resources: OnceCell<TextureResources>,
}
impl Texture {
    pub const DEPTH_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Depth32Float;

    pub fn depth(label: &str) -> Self {
        Self {
            image: TextureImage::Depth,
            label: label.into(),
            resources: OnceCell::new(),
        }
    }

    pub fn from_image(image: image::DynamicImage, label: &str, kind: ImageTextureKind) -> Self {
        let rgba = image.to_rgba8();
        let dimensions = image.dimensions();

        let size = wgpu::Extent3d {
            width: dimensions.0,
            height: dimensions.1,
            depth_or_array_layers: 1,
        };

        Self {
            image: match kind {
                ImageTextureKind::Diffuse => TextureImage::Diffuse(image),
                ImageTextureKind::NormalMap => TextureImage::NormalMap(image),
            },
            label: label.into(),
            resources: OnceCell::new(),
        }
    }

    pub fn from_bytes(bytes: &[u8], label: &str, kind: ImageTextureKind) -> Self {
        let image = image::load_from_memory(bytes).unwrap();
        Self::from_image(image, label, kind)
    }

    pub fn resources(
        &self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        size: (u32, u32),
    ) -> &TextureResources {
        self.resources.get_or_init(|| match &self.image {
            TextureImage::Depth => Self::create_depth_texture(device, size, &self.label),
            TextureImage::Diffuse(image) => Self::create_image_texture(
                device,
                queue,
                &self.label,
                image,
                ImageTextureKind::Diffuse,
            ),
            TextureImage::NormalMap(image) => Self::create_image_texture(
                device,
                queue,
                &self.label,
                image,
                ImageTextureKind::NormalMap,
            ),
        })
    }

    fn create_depth_texture(
        device: &wgpu::Device,
        size: (u32, u32),
        label: &str,
    ) -> TextureResources {
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some(label),
            size: wgpu::Extent3d {
                width: size.0,
                height: size.1,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Depth32Float,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        });

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());

        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::FilterMode::Nearest,
            compare: Some(wgpu::CompareFunction::LessEqual),
            lod_min_clamp: 0.0,
            lod_max_clamp: 100.0,
            ..Default::default()
        });

        TextureResources {
            texture: Arc::new(texture),
            view: Arc::new(view),
            sampler: Arc::new(sampler),
        }
    }

    fn create_image_texture(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        label: &str,
        image: &image::DynamicImage,
        kind: ImageTextureKind,
    ) -> TextureResources {
        let format = match kind {
            ImageTextureKind::Diffuse => wgpu::TextureFormat::Rgba8UnormSrgb,
            ImageTextureKind::NormalMap => wgpu::TextureFormat::Rgba8Unorm,
        };

        let dims = image.dimensions();

        let size = wgpu::Extent3d {
            width: dims.0,
            height: dims.1,
            depth_or_array_layers: 1,
        };

        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some(label),
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: format,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });

        queue.write_texture(
            wgpu::ImageCopyTexture {
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            &image.to_rgba8(),
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(4 * dims.0),
                rows_per_image: Some(dims.1),
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

        TextureResources {
            texture: Arc::new(texture),
            view: Arc::new(view),
            sampler: Arc::new(sampler),
        }
    }
}
