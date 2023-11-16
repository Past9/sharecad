use std::cell::OnceCell;

use image::{DynamicImage, GenericImage, GenericImageView};
use rand::Rng;
use space::{vec3, Vec3};

use crate::{
    color::{rgb, Rgb},
    texture::TextureId,
};

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
    pub emissive: TextureId,
    pub roughness: TextureId,
    pub metallic: TextureId,
    pub ambient: TextureId,
    pub transmit: TextureId,
    pub is_translucent: bool,
}
impl Material {
    pub fn new(
        id: MaterialId,
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
            id,
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
pub struct MaterialSpec {
    pub diffuse: RgbSpec,
    pub transmit: RgbSpec,
    pub normal: Vec3Spec,
    pub emissive: RgbSpec,
    pub roughness: RgbSpec,
    pub metallic: RgbSpec,
    pub ambient: RgbSpec,

    is_translucent: OnceCell<bool>,
}
impl MaterialSpec {
    pub fn is_translucent(&self) -> bool {
        *self
            .is_translucent
            .get_or_init(|| !self.transmit.all_approx_one_or_zero())
    }

    pub fn diffuse(mut self, diffuse: RgbSpec) -> Self {
        self.diffuse = diffuse;
        self
    }

    pub fn transmit(mut self, transmit: RgbSpec) -> Self {
        self.transmit = transmit;
        self
    }

    pub fn normal(mut self, normal: Vec3Spec) -> Self {
        self.normal = normal;
        self
    }

    pub fn emissive(mut self, emissive: RgbSpec) -> Self {
        self.emissive = emissive;
        self
    }

    pub fn roughness(mut self, roughness: RgbSpec) -> Self {
        self.roughness = roughness;
        self
    }

    pub fn metallic(mut self, metallic: RgbSpec) -> Self {
        self.metallic = metallic;
        self
    }

    pub fn ambient(mut self, ambient: RgbSpec) -> Self {
        self.ambient = ambient;
        self
    }

    pub fn diffuse_rgb(mut self, rgb: Rgb) -> Self {
        self.diffuse = RgbSpec::Rgb(rgb);
        self
    }

    pub fn transmit_rgb(mut self, rgb: Rgb) -> Self {
        self.transmit = RgbSpec::Rgb(rgb);
        self
    }

    pub fn normal_vec(mut self, vec: Vec3) -> Self {
        self.normal = Vec3Spec::Vec3(vec);
        self
    }

    pub fn emissive_rgb(mut self, rgb: Rgb) -> Self {
        self.emissive = RgbSpec::Rgb(rgb);
        self
    }

    pub fn roughness_rgb(mut self, rgb: Rgb) -> Self {
        self.roughness = RgbSpec::Rgb(rgb);
        self
    }

    pub fn metallic_rgb(mut self, rgb: Rgb) -> Self {
        self.metallic = RgbSpec::Rgb(rgb);
        self
    }

    pub fn ambient_rgb(mut self, rgb: Rgb) -> Self {
        self.ambient = RgbSpec::Rgb(rgb);
        self
    }

    pub fn diffuse_from_image(self, image: image::DynamicImage) -> Self {
        self.diffuse(RgbSpec::Texture(image))
    }

    pub fn diffuse_from_bytes(self, bytes: &[u8]) -> Self {
        let image = image::load_from_memory(bytes).unwrap();
        self.diffuse_from_image(image)
    }

    pub fn diffuse_from_file(self, path: &str) -> Self {
        self.diffuse_from_bytes(&std::fs::read(path).unwrap())
    }

    pub fn transmit_from_image(self, image: image::DynamicImage) -> Self {
        self.transmit(RgbSpec::Texture(image))
    }

    pub fn transmit_from_bytes(self, bytes: &[u8]) -> Self {
        let image = image::load_from_memory(bytes).unwrap();
        self.transmit_from_image(image)
    }

    pub fn transmit_from_file(self, path: &str) -> Self {
        self.transmit_from_bytes(&std::fs::read(path).unwrap())
    }

    pub fn normal_from_image(self, image: image::DynamicImage) -> Self {
        self.normal(Vec3Spec::Map(image))
    }

    pub fn normal_from_bytes(self, bytes: &[u8]) -> Self {
        let image = image::load_from_memory(bytes).unwrap();
        self.normal_from_image(image)
    }

    pub fn normal_from_file(self, path: &str) -> Self {
        self.normal_from_bytes(&std::fs::read(path).unwrap())
    }

    pub fn emissive_from_image(self, image: image::DynamicImage) -> Self {
        self.emissive(RgbSpec::Texture(image))
    }

    pub fn emissive_from_bytes(self, bytes: &[u8]) -> Self {
        let image = image::load_from_memory(bytes).unwrap();
        self.emissive_from_image(image)
    }

    pub fn emissive_from_file(self, path: &str) -> Self {
        self.emissive_from_bytes(&std::fs::read(path).unwrap())
    }

    pub fn roughness_from_image(self, image: image::DynamicImage) -> Self {
        self.roughness(RgbSpec::Texture(image))
    }

    pub fn roughness_from_bytes(self, bytes: &[u8]) -> Self {
        let image = image::load_from_memory(bytes).unwrap();
        self.roughness_from_image(image)
    }

    pub fn roughness_from_file(self, path: &str) -> Self {
        self.roughness_from_bytes(&std::fs::read(path).unwrap())
    }

    pub fn metallic_from_image(self, image: image::DynamicImage) -> Self {
        self.metallic(RgbSpec::Texture(image))
    }

    pub fn metallic_from_bytes(self, bytes: &[u8]) -> Self {
        let image = image::load_from_memory(bytes).unwrap();
        self.metallic_from_image(image)
    }

    pub fn metallic_from_file(self, path: &str) -> Self {
        self.metallic_from_bytes(&std::fs::read(path).unwrap())
    }

    pub fn ambient_from_image(self, image: image::DynamicImage) -> Self {
        self.ambient(RgbSpec::Texture(image))
    }

    pub fn ambient_from_bytes(self, bytes: &[u8]) -> Self {
        let image = image::load_from_memory(bytes).unwrap();
        self.ambient_from_image(image)
    }

    pub fn ambient_from_file(self, path: &str) -> Self {
        self.ambient_from_bytes(&std::fs::read(path).unwrap())
    }
}
impl Default for MaterialSpec {
    fn default() -> Self {
        Self {
            diffuse: RgbSpec::default_diffuse(),
            transmit: RgbSpec::default_transmit(),
            normal: Vec3Spec::default_normal(),
            emissive: RgbSpec::default_emissive(),
            roughness: RgbSpec::default_roughness(),
            metallic: RgbSpec::default_metallic(),
            ambient: RgbSpec::default_ambient(),

            is_translucent: OnceCell::new(),
        }
    }
}

#[derive(Debug)]
pub enum RgbSpec {
    Texture(image::DynamicImage),
    Rgb(Rgb),
}
impl RgbSpec {
    pub fn all_approx_one_or_zero(&self) -> bool {
        match self {
            RgbSpec::Texture(image) => {
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
            RgbSpec::Rgb(rgb) => rgb.all_approx_one_or_zero(),
        }
    }

    pub fn image(&self) -> image::DynamicImage {
        match self {
            RgbSpec::Texture(image) => image.clone(),
            RgbSpec::Rgb(rgb) => rgb.create_image(),
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
        let mut rng = rand::thread_rng();
        let r: f32 = rng.gen::<f32>() * 0.5 + 0.5;
        let g: f32 = rng.gen::<f32>() * 0.5 + 0.5;
        let b: f32 = rng.gen::<f32>() * 0.5 + 0.5;
        Self::Rgb(rgb(r, g, b))
        //Self::Rgb(rgb(0.0, 0.0, 0.0))
    }
}

#[derive(Debug)]
pub enum Vec3Spec {
    Map(image::DynamicImage),
    Vec3(Vec3),
}
impl Vec3Spec {
    pub fn image(&self) -> image::DynamicImage {
        match self {
            Vec3Spec::Map(image) => image.clone(),
            Vec3Spec::Vec3(vec3) => Rgb::from_normal_vec(*vec3).create_image(),
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
