#[macro_use]
mod tolerance;

mod angle;
mod coord2;
mod mat22;
mod mat33;
mod mat44;
mod point2;
mod point3;
mod quat;
mod vec2;
mod vec3;
mod vec4;

pub use angle::*;
pub use coord2::*;
pub use mat22::*;
pub use mat33::*;
pub use mat44::*;
pub use point2::*;
pub use point3::*;
pub use quat::*;
pub use tolerance::*;
pub use vec2::*;
pub use vec3::*;
pub use vec4::*;

pub fn lerp(a: f64, b: f64, t: f64) -> f64 {
    (1.0 - t) * a + t * b
}
