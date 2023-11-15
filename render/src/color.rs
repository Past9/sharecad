use image::{DynamicImage, GenericImage};
use space::{vec3, Vec3};

#[derive(Debug, Clone)]
pub struct Rgba {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}
impl Rgba {
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }

    pub fn clamped(self) -> Self {
        Self {
            r: self.r.clamp(0.0, 1.0),
            g: self.g.clamp(0.0, 1.0),
            b: self.b.clamp(0.0, 1.0),
            a: self.a.clamp(0.0, 1.0),
        }
    }

    /// Create a small texture image filled with this color.
    pub fn create_image(&self) -> image::DynamicImage {
        let mut image = DynamicImage::new_rgb8(2, 2);
        image.put_pixel(0, 0, image::Rgba::from(self.as_u8s()));
        image.put_pixel(0, 1, image::Rgba::from(self.as_u8s()));
        image.put_pixel(1, 0, image::Rgba::from(self.as_u8s()));
        image.put_pixel(1, 1, image::Rgba::from(self.as_u8s()));
        image
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

#[derive(Debug, Clone)]
pub struct Rgb {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}
impl Rgb {
    pub fn all_approx_one_or_zero(&self) -> bool {
        const ZERO_THRESHOLD: f32 = 0.5 / 255.0;
        const ONE_THRESHOLD: f32 = 254.5 / 255.0;
        !(self.r > ZERO_THRESHOLD
            || self.r < ONE_THRESHOLD
            || self.g > ZERO_THRESHOLD
            || self.g < ONE_THRESHOLD
            || self.b > ZERO_THRESHOLD
            || self.b < ONE_THRESHOLD)
    }

    pub fn new(r: f32, g: f32, b: f32) -> Self {
        Self { r, g, b }
    }

    pub fn clamped(self) -> Self {
        Self {
            r: self.r.clamp(0.0, 1.0),
            g: self.g.clamp(0.0, 1.0),
            b: self.b.clamp(0.0, 1.0),
        }
    }

    pub fn create_image(&self) -> image::DynamicImage {
        self.with_a(1.0).create_image()
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

    pub fn as_f32s(&self) -> [f32; 3] {
        [self.r, self.g, self.b]
    }

    fn f32_to_u8(val: f32) -> u8 {
        (val * 255.0).round() as u8
    }

    /// Convert a normal vector (as from a normal map texture) into RGB
    /// colorspace.
    pub fn from_normal_vec(vec: Vec3) -> Self {
        let rgb_vec = (vec + vec3(1.0, 1.0, 1.0)) / 2.0;
        Self::new(rgb_vec.x as f32, rgb_vec.y as f32, rgb_vec.z as f32)
    }
}
pub fn rgb(r: f32, g: f32, b: f32) -> Rgb {
    Rgb::new(r, g, b)
}
