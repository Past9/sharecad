use crate::{
    color::{rgb, Rgb},
    texture::TextureId,
};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct CurveMaterialId(pub u32);
impl From<u32> for CurveMaterialId {
    fn from(id: u32) -> Self {
        CurveMaterialId(id)
    }
}
impl From<CurveMaterialId> for u32 {
    fn from(id: CurveMaterialId) -> Self {
        id.0
    }
}

#[derive(Debug, PartialEq)]
pub struct CurveMaterial {
    pub color: TextureId,
}
impl CurveMaterial {
    pub fn new(color: TextureId) -> Self {
        Self { color }
    }
}

#[derive(Debug)]
pub struct CurveMaterialSpec {
    pub color: CurveRgbSpec,
}
impl CurveMaterialSpec {
    pub fn color(mut self, color: CurveRgbSpec) -> Self {
        self.color = color;
        self
    }

    pub fn color_rgb(mut self, color: Rgb) -> Self {
        self.color = CurveRgbSpec::Rgb(color);
        self
    }

    pub fn color_from_image(self, image: image::DynamicImage) -> Self {
        self.color(CurveRgbSpec::Texture(image))
    }

    pub fn color_from_bytes(self, bytes: &[u8]) -> Self {
        let image = image::load_from_memory(bytes).unwrap();
        self.color_from_image(image)
    }

    pub fn color_from_file(self, path: &str) -> Self {
        self.color_from_bytes(&std::fs::read(path).unwrap())
    }
}
impl Default for CurveMaterialSpec {
    fn default() -> Self {
        Self {
            color: CurveRgbSpec::default_color(),
        }
    }
}

#[derive(Debug)]
pub enum CurveRgbSpec {
    Texture(image::DynamicImage),
    Rgb(Rgb),
}
impl CurveRgbSpec {
    pub fn image(&self) -> image::DynamicImage {
        match self {
            CurveRgbSpec::Texture(image) => image.clone(),
            CurveRgbSpec::Rgb(rgb) => rgb.create_image(),
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
