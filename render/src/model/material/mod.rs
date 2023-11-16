mod curve;
mod surface;

use crate::color::{rgb, Rgb};
use image::GenericImageView;
use space::{vec3, Vec3};

pub use curve::*;
pub use surface::*;

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
