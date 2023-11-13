use image::GenericImageView;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct TextureId(pub u32);
impl From<u32> for TextureId {
    fn from(id: u32) -> Self {
        TextureId(id)
    }
}

#[derive(Debug)]
pub struct Texture {
    pub id: TextureId,
    pub image: TextureImage,
}
impl Texture {
    pub fn from_image(id: TextureId, image: image::DynamicImage, kind: ImageTextureKind) -> Self {
        Self {
            id,
            image: match kind {
                ImageTextureKind::Diffuse => TextureImage::Rgb(image),
                ImageTextureKind::NormalMap => TextureImage::Vector(image),
            },
        }
    }

    pub fn from_bytes(id: TextureId, bytes: &[u8], kind: ImageTextureKind) -> Self {
        let image = image::load_from_memory(bytes).unwrap();
        Self::from_image(id, image, kind)
    }
}

pub enum TextureImage {
    Rgb(image::DynamicImage),
    Vector(image::DynamicImage),
}
impl std::fmt::Debug for TextureImage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Rgb(image) => {
                let dims = image.dimensions();
                f.write_fmt(format_args!("Diffuse(<{}x{} image>)", dims.0, dims.1))
            }
            Self::Vector(image) => {
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
