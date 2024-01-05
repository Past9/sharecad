mod bsp;
mod curve;
mod surface;

pub use curve::*;
pub use surface::*;

use crate::math::Angle;

pub enum TessellationTolerance {
    Distance(f64),
    Angle(Angle),
    DistanceAndAngle(f64, Angle),
}
