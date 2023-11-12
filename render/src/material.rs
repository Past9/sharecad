use image::{DynamicImage, GenericImage};
use space::{vec3, Vec3};

use crate::texture::TextureId;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct MaterialId(pub u32);
impl From<u32> for MaterialId {
    fn from(id: u32) -> Self {
        MaterialId(id)
    }
}

#[derive(Debug)]
pub struct Material {
    pub id: MaterialId,
    pub diffuse: TextureId,
    pub normal: TextureId,
}
impl Material {
    pub fn new(id: MaterialId, diffuse: TextureId, normal: TextureId) -> Self {
        Self {
            id,
            diffuse,
            normal,
        }
    }
}

pub struct MaterialSpec {
    pub diffuse: DiffuseSpec,
    pub normal: NormalSpec,
}
impl MaterialSpec {
    pub fn diffuse(self, diffuse: DiffuseSpec) -> Self {
        let Self { normal, .. } = self;
        Self { diffuse, normal }
    }

    pub fn normal(self, normal: NormalSpec) -> Self {
        let Self { diffuse, .. } = self;
        Self { diffuse, normal }
    }

    pub fn diffuse_rgba(self, rgba: Rgba) -> Self {
        let Self { normal, .. } = self;

        Self {
            diffuse: DiffuseSpec::Rgba(rgba),
            normal,
        }
    }

    pub fn normal_vec(self, vec: Vec3) -> Self {
        let Self { diffuse, .. } = self;

        Self {
            normal: NormalSpec::Vec3(vec),
            diffuse,
        }
    }

    pub fn diffuse_from_image(self, image: image::DynamicImage) -> Self {
        self.diffuse(DiffuseSpec::Texture(image))
    }

    pub fn diffuse_from_bytes(self, bytes: &[u8]) -> Self {
        let image = image::load_from_memory(bytes).unwrap();
        self.diffuse_from_image(image)
    }

    pub fn diffuse_from_file(self, path: &str) -> Self {
        self.diffuse_from_bytes(&std::fs::read(path).unwrap())
    }

    pub fn normal_from_image(self, image: image::DynamicImage) -> Self {
        self.normal(NormalSpec::Map(image))
    }

    pub fn normal_from_bytes(self, bytes: &[u8]) -> Self {
        let image = image::load_from_memory(bytes).unwrap();
        self.normal_from_image(image)
    }

    pub fn normal_from_file(self, path: &str) -> Self {
        self.normal_from_bytes(&std::fs::read(path).unwrap())
    }
}
impl Default for MaterialSpec {
    fn default() -> Self {
        Self {
            diffuse: Default::default(),
            normal: Default::default(),
        }
    }
}

pub enum DiffuseSpec {
    Texture(image::DynamicImage),
    Rgba(Rgba),
}
impl DiffuseSpec {
    pub fn image(&self) -> image::DynamicImage {
        match self {
            DiffuseSpec::Texture(image) => image.clone(),
            DiffuseSpec::Rgba(rgba) => {
                let mut image = DynamicImage::new_rgb8(2, 2);
                image.put_pixel(0, 0, image::Rgba::from(rgba.as_u8s()));
                image.put_pixel(0, 1, image::Rgba::from(rgba.as_u8s()));
                image.put_pixel(1, 0, image::Rgba::from(rgba.as_u8s()));
                image.put_pixel(1, 1, image::Rgba::from(rgba.as_u8s()));
                image
            }
        }
    }
}
impl Default for DiffuseSpec {
    fn default() -> Self {
        Self::Rgba(rgba(0.5, 0.5, 0.5, 1.0))
    }
}

pub enum NormalSpec {
    Map(image::DynamicImage),
    Vec3(Vec3),
}
impl NormalSpec {
    pub fn image(&self) -> image::DynamicImage {
        match self {
            NormalSpec::Map(image) => image.clone(),
            NormalSpec::Vec3(vec3) => {
                let mut image = DynamicImage::new_rgb8(2, 2);
                let color = rgb(vec3.x as f32, vec3.y as f32, vec3.z as f32).with_a(1.0);
                image.put_pixel(0, 0, image::Rgba::from(color.as_u8s()));
                image.put_pixel(0, 1, image::Rgba::from(color.as_u8s()));
                image.put_pixel(1, 0, image::Rgba::from(color.as_u8s()));
                image.put_pixel(1, 1, image::Rgba::from(color.as_u8s()));
                image
            }
        }
    }
}
impl Default for NormalSpec {
    fn default() -> Self {
        Self::Vec3(vec3(0.5, 0.5, 1.0))
    }
}

#[derive(Debug)]
pub struct Rgba {
    r: f32,
    g: f32,
    b: f32,
    a: f32,
}
impl Rgba {
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }

    pub fn as_u8s(&self) -> [u8; 4] {
        [
            Self::f32_to_u8(self.r),
            Self::f32_to_u8(self.g),
            Self::f32_to_u8(self.b),
            Self::f32_to_u8(self.a),
        ]
    }

    fn f32_to_u8(val: f32) -> u8 {
        (val * 255.0).round() as u8
    }
}
pub fn rgba(r: f32, g: f32, b: f32, a: f32) -> Rgba {
    Rgba::new(r, g, b, a)
}

#[derive(Debug)]
pub struct Rgb {
    r: f32,
    g: f32,
    b: f32,
}
impl Rgb {
    pub fn new(r: f32, g: f32, b: f32) -> Self {
        Self { r, g, b }
    }

    pub fn with_a(&self, a: f32) -> Rgba {
        rgba(self.r, self.g, self.b, a)
    }

    pub fn as_u8s(&self) -> [u8; 3] {
        [
            Self::f32_to_u8(self.r),
            Self::f32_to_u8(self.g),
            Self::f32_to_u8(self.b),
        ]
    }

    fn f32_to_u8(val: f32) -> u8 {
        (val * 255.0).round() as u8
    }
}
pub fn rgb(r: f32, g: f32, b: f32) -> Rgb {
    Rgb::new(r, g, b)
}
