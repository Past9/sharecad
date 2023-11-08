use image::GenericImageView;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct TextureId(pub u32);
impl From<u32> for TextureId {
    fn from(id: u32) -> Self {
        TextureId(id)
    }
}

pub enum TextureImage {
    Diffuse(image::DynamicImage),
    NormalMap(image::DynamicImage),
}
impl std::fmt::Debug for TextureImage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
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
pub struct Texture {
    pub id: TextureId,
    pub image: TextureImage,
    pub label: String,
}
impl Texture {
    pub fn from_image(
        id: TextureId,
        image: image::DynamicImage,
        label: &str,
        kind: ImageTextureKind,
    ) -> Self {
        Self {
            id,
            image: match kind {
                ImageTextureKind::Diffuse => TextureImage::Diffuse(image),
                ImageTextureKind::NormalMap => TextureImage::NormalMap(image),
            },
            label: label.into(),
        }
    }

    pub fn from_bytes(id: TextureId, bytes: &[u8], label: &str, kind: ImageTextureKind) -> Self {
        let image = image::load_from_memory(bytes).unwrap();
        Self::from_image(id, image, label, kind)
    }
}
