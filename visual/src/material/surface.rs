use std::cell::OnceCell;

use geometry::math::{vec3, Vec3};
use image::GenericImageView;

use crate::{
    color::{rgb, Rgb},
    texture::TextureId,
};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct SurfaceMaterialId(pub u32);
impl From<u32> for SurfaceMaterialId {
    fn from(id: u32) -> Self {
        SurfaceMaterialId(id)
    }
}
impl From<SurfaceMaterialId> for u32 {
    fn from(id: SurfaceMaterialId) -> Self {
        id.0
    }
}

pub trait DefaultSurfaceMaterials {
    fn copper_polished() -> SurfaceMaterialSpec;
}

#[derive(Debug, PartialEq)]
pub struct SurfaceMaterial {
    pub diffuse: TextureId,
    pub normal: TextureId,
    pub emissive: TextureId,
    pub roughness: TextureId,
    pub metallic: TextureId,
    pub ambient: TextureId,
    pub transmit: TextureId,
    pub is_translucent: bool,
}
impl SurfaceMaterial {
    pub fn new(
        diffuse: TextureId,
        normal: TextureId,
        emissive: TextureId,
        roughness: TextureId,
        metallic: TextureId,
        ambient: TextureId,
        transmit: TextureId,
        is_translucent: bool,
    ) -> Self {
        Self {
            diffuse,
            normal,
            emissive,
            roughness,
            metallic,
            ambient,
            transmit,
            is_translucent,
        }
    }
}

#[derive(Debug)]
pub struct SurfaceMaterialSpec {
    pub diffuse: SurfaceRgbSpec,
    pub transmit: SurfaceRgbSpec,
    pub normal: SurfaceVec3Spec,
    pub emissive: SurfaceRgbSpec,
    pub roughness: SurfaceRgbSpec,
    pub metallic: SurfaceRgbSpec,
    pub ambient: SurfaceRgbSpec,

    is_translucent: OnceCell<bool>,
}
impl SurfaceMaterialSpec {
    pub fn is_translucent(&self) -> bool {
        *self
            .is_translucent
            .get_or_init(|| !self.transmit.all_approx_one_or_zero())
    }

    pub fn diffuse(mut self, diffuse: SurfaceRgbSpec) -> Self {
        self.diffuse = diffuse;
        self
    }

    pub fn transmit(mut self, transmit: SurfaceRgbSpec) -> Self {
        self.transmit = transmit;
        self
    }

    pub fn normal(mut self, normal: SurfaceVec3Spec) -> Self {
        self.normal = normal;
        self
    }

    pub fn emissive(mut self, emissive: SurfaceRgbSpec) -> Self {
        self.emissive = emissive;
        self
    }

    pub fn roughness(mut self, roughness: SurfaceRgbSpec) -> Self {
        self.roughness = roughness;
        self
    }

    pub fn metallic(mut self, metallic: SurfaceRgbSpec) -> Self {
        self.metallic = metallic;
        self
    }

    pub fn ambient(mut self, ambient: SurfaceRgbSpec) -> Self {
        self.ambient = ambient;
        self
    }

    pub fn diffuse_rgb(mut self, rgb: Rgb) -> Self {
        self.diffuse = SurfaceRgbSpec::Rgb(rgb);
        self
    }

    pub fn transmit_rgb(mut self, rgb: Rgb) -> Self {
        self.transmit = SurfaceRgbSpec::Rgb(rgb);
        self
    }

    pub fn normal_vec(mut self, vec: Vec3<f64>) -> Self {
        self.normal = SurfaceVec3Spec::Vec3(vec);
        self
    }

    pub fn emissive_rgb(mut self, rgb: Rgb) -> Self {
        self.emissive = SurfaceRgbSpec::Rgb(rgb);
        self
    }

    pub fn roughness_rgb(mut self, rgb: Rgb) -> Self {
        self.roughness = SurfaceRgbSpec::Rgb(rgb);
        self
    }

    pub fn metallic_rgb(mut self, rgb: Rgb) -> Self {
        self.metallic = SurfaceRgbSpec::Rgb(rgb);
        self
    }

    pub fn ambient_rgb(mut self, rgb: Rgb) -> Self {
        self.ambient = SurfaceRgbSpec::Rgb(rgb);
        self
    }

    pub fn diffuse_from_image(self, image: image::DynamicImage) -> Self {
        self.diffuse(SurfaceRgbSpec::Texture(image))
    }

    pub fn diffuse_from_bytes(self, bytes: &[u8]) -> Self {
        let image = image::load_from_memory(bytes).unwrap();
        self.diffuse_from_image(image)
    }

    pub fn diffuse_from_file(self, path: &str) -> Self {
        self.diffuse_from_bytes(&std::fs::read(path).unwrap())
    }

    pub fn transmit_from_image(self, image: image::DynamicImage) -> Self {
        self.transmit(SurfaceRgbSpec::Texture(image))
    }

    pub fn transmit_from_bytes(self, bytes: &[u8]) -> Self {
        let image = image::load_from_memory(bytes).unwrap();
        self.transmit_from_image(image)
    }

    pub fn transmit_from_file(self, path: &str) -> Self {
        self.transmit_from_bytes(&std::fs::read(path).unwrap())
    }

    pub fn normal_from_image(self, image: image::DynamicImage) -> Self {
        self.normal(SurfaceVec3Spec::Map(image))
    }

    pub fn normal_from_bytes(self, bytes: &[u8]) -> Self {
        let image = image::load_from_memory(bytes).unwrap();
        self.normal_from_image(image)
    }

    pub fn normal_from_file(self, path: &str) -> Self {
        self.normal_from_bytes(&std::fs::read(path).unwrap())
    }

    pub fn emissive_from_image(self, image: image::DynamicImage) -> Self {
        self.emissive(SurfaceRgbSpec::Texture(image))
    }

    pub fn emissive_from_bytes(self, bytes: &[u8]) -> Self {
        let image = image::load_from_memory(bytes).unwrap();
        self.emissive_from_image(image)
    }

    pub fn emissive_from_file(self, path: &str) -> Self {
        self.emissive_from_bytes(&std::fs::read(path).unwrap())
    }

    pub fn roughness_from_image(self, image: image::DynamicImage) -> Self {
        self.roughness(SurfaceRgbSpec::Texture(image))
    }

    pub fn roughness_from_bytes(self, bytes: &[u8]) -> Self {
        let image = image::load_from_memory(bytes).unwrap();
        self.roughness_from_image(image)
    }

    pub fn roughness_from_file(self, path: &str) -> Self {
        self.roughness_from_bytes(&std::fs::read(path).unwrap())
    }

    pub fn metallic_from_image(self, image: image::DynamicImage) -> Self {
        self.metallic(SurfaceRgbSpec::Texture(image))
    }

    pub fn metallic_from_bytes(self, bytes: &[u8]) -> Self {
        let image = image::load_from_memory(bytes).unwrap();
        self.metallic_from_image(image)
    }

    pub fn metallic_from_file(self, path: &str) -> Self {
        self.metallic_from_bytes(&std::fs::read(path).unwrap())
    }

    pub fn ambient_from_image(self, image: image::DynamicImage) -> Self {
        self.ambient(SurfaceRgbSpec::Texture(image))
    }

    pub fn ambient_from_bytes(self, bytes: &[u8]) -> Self {
        let image = image::load_from_memory(bytes).unwrap();
        self.ambient_from_image(image)
    }

    pub fn ambient_from_file(self, path: &str) -> Self {
        self.ambient_from_bytes(&std::fs::read(path).unwrap())
    }

    pub fn matte(self) -> Self {
        self.roughness_rgb(rgb(1.0, 1.0, 1.0))
    }

    pub fn eggshell(self) -> Self {
        self.roughness_rgb(rgb(0.8, 0.8, 0.8))
    }

    pub fn semigloss(self) -> Self {
        self.roughness_rgb(rgb(0.35, 0.35, 0.35))
    }

    pub fn gloss(self) -> Self {
        self.roughness_rgb(rgb(0.2, 0.2, 0.2))
    }

    pub fn metal(self) -> Self {
        self.metallic_rgb(rgb(0.7, 0.7, 0.7))
    }

    pub fn copper(self) -> Self {
        self.diffuse_rgb(rgb(0.72, 0.45, 0.2))
    }

    pub fn color(self, rgb: Rgb) -> Self {
        self.diffuse_rgb(rgb)
    }
}
impl Default for SurfaceMaterialSpec {
    fn default() -> Self {
        Self {
            diffuse: SurfaceRgbSpec::default_diffuse(),
            transmit: SurfaceRgbSpec::default_transmit(),
            normal: SurfaceVec3Spec::default_normal(),
            emissive: SurfaceRgbSpec::default_emissive(),
            roughness: SurfaceRgbSpec::default_roughness(),
            metallic: SurfaceRgbSpec::default_metallic(),
            ambient: SurfaceRgbSpec::default_ambient(),

            is_translucent: OnceCell::new(),
        }
    }
}

#[derive(Debug)]
pub enum SurfaceRgbSpec {
    Texture(image::DynamicImage),
    Rgb(Rgb),
}
impl SurfaceRgbSpec {
    pub fn all_approx_one_or_zero(&self) -> bool {
        match self {
            SurfaceRgbSpec::Texture(image) => {
                for x in 0..image.width() {
                    for y in 0..image.height() {
                        let pixel = image.get_pixel(x, y);
                        if (pixel[0] != 0 && pixel[0] != 255)
                            || (pixel[1] != 0 && pixel[1] != 255)
                            || (pixel[2] != 0 && pixel[2] != 255)
                        {
                            return false;
                        }
                    }
                }

                true
            }
            SurfaceRgbSpec::Rgb(rgb) => rgb.all_approx_one_or_zero(),
        }
    }

    pub fn image(&self) -> image::DynamicImage {
        match self {
            SurfaceRgbSpec::Texture(image) => image.clone(),
            SurfaceRgbSpec::Rgb(rgb) => rgb.create_image(),
        }
    }

    pub fn from_image(image: image::DynamicImage) -> Self {
        Self::Texture(image)
    }

    pub fn from_bytes(bytes: &[u8]) -> Self {
        let image = image::load_from_memory(bytes).unwrap();
        Self::from_image(image)
    }

    pub fn from_file(path: &str) -> Self {
        Self::from_bytes(&std::fs::read(path).unwrap())
    }

    pub fn default_emissive() -> Self {
        Self::Rgb(rgb(0.0, 0.0, 0.0))
    }

    pub fn default_roughness() -> Self {
        Self::Rgb(rgb(0.2, 0.2, 0.2))
    }

    pub fn default_metallic() -> Self {
        Self::Rgb(rgb(0.0, 0.0, 0.0))
    }

    pub fn default_ambient() -> Self {
        Self::Rgb(rgb(1.0, 1.0, 1.0))
    }

    pub fn default_diffuse() -> Self {
        Self::Rgb(rgb(0.5, 0.5, 0.5))
    }

    pub fn default_transmit() -> Self {
        /*
        let mut rng = rand::thread_rng();
        let r: f32 = rng.gen::<f32>() * 0.5 + 0.5;
        let g: f32 = rng.gen::<f32>() * 0.5 + 0.5;
        let b: f32 = rng.gen::<f32>() * 0.5 + 0.5;
        Self::Rgb(rgb(r, g, b))
        */
        Self::Rgb(rgb(0.0, 0.0, 0.0))
    }
}

#[derive(Debug)]
pub enum SurfaceVec3Spec {
    Map(image::DynamicImage),
    Vec3(Vec3<f64>),
}
impl SurfaceVec3Spec {
    pub fn image(&self) -> image::DynamicImage {
        match self {
            SurfaceVec3Spec::Map(image) => image.clone(),
            SurfaceVec3Spec::Vec3(vec3) => Rgb::from_normal_vec(*vec3).create_image(),
        }
    }

    pub fn from_image(image: image::DynamicImage) -> Self {
        Self::Map(image)
    }

    pub fn from_bytes(bytes: &[u8]) -> Self {
        let image = image::load_from_memory(bytes).unwrap();
        Self::from_image(image)
    }

    pub fn from_file(path: &str) -> Self {
        Self::from_bytes(&std::fs::read(path).unwrap())
    }

    pub fn default_normal() -> Self {
        Self::Vec3(vec3(0.0, 0.0, 1.0))
    }
}
