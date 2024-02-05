mod bsp;
mod curve;
mod surface;

pub use bsp::*;
pub use curve::*;
pub use surface::*;

use crate::math::{Angle, Scalar};

pub enum TessellationTolerance {
    Distance(f64),
    Angle(Angle<f64>),
    DistanceAndAngle(f64, Angle<f64>),
}
