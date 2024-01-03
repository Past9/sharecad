use crate::{
    color::{rgb, Rgb},
    texture::TextureId,
};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct PointMaterialId(pub u32);
impl From<u32> for PointMaterialId {
    fn from(id: u32) -> Self {
        PointMaterialId(id)
    }
}
impl From<PointMaterialId> for u32 {
    fn from(id: PointMaterialId) -> Self {
        id.0
    }
}

#[derive(Debug, PartialEq)]
pub struct PointMaterial {
    pub color: TextureId,
}
impl PointMaterial {
    pub fn new(color: TextureId) -> Self {
        Self { color }
    }
}

#[derive(Debug)]
pub struct PointMaterialSpec {
    pub color: PointRgbSpec,
}
impl PointMaterialSpec {
    pub fn color(mut self, color: PointRgbSpec) -> Self {
        self.color = color;
        self
    }

    pub fn color_rgb(mut self, color: Rgb) -> Self {
        self.color = PointRgbSpec::Rgb(color);
        self
    }

    pub fn color_from_image(self, image: image::DynamicImage) -> Self {
        self.color(PointRgbSpec::Texture(image))
    }

    pub fn color_from_bytes(self, bytes: &[u8]) -> Self {
        let image = image::load_from_memory(bytes).unwrap();
        self.color_from_image(image)
    }

    pub fn color_from_file(self, path: &str) -> Self {
        self.color_from_bytes(&std::fs::read(path).unwrap())
    }
}
impl Default for PointMaterialSpec {
    fn default() -> Self {
        Self {
            color: PointRgbSpec::default_color(),
        }
    }
}

#[derive(Debug)]
pub enum PointRgbSpec {
    Texture(image::DynamicImage),
    Rgb(Rgb),
}
impl PointRgbSpec {
    pub fn image(&self) -> image::DynamicImage {
        match self {
            PointRgbSpec::Texture(image) => image.clone(),
            PointRgbSpec::Rgb(rgb) => rgb.create_image(),
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

    pub fn default_color() -> Self {
        Self::Rgb(rgb(0.0, 0.0, 0.0))
    }
}
