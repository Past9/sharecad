mod float;
mod interval;
//mod point2;
mod vec2;

//use std::ops::{Add, Div, Mul, Neg, Sub};

use std::ops::{Add, Div, Mul, Neg, Sub};

pub use float::*;
pub use interval::*;
//pub use point2::*;
pub use vec2::*;

pub trait Scalar
where
    Self: std::fmt::Display
        + Copy
        + Clone
        + Neg<Output = Self>
        + Add<Self, Output = Self>
        + Sub<Self, Output = Self>
        + Mul<Self, Output = Self>
        + Div<Self, Output = Self>,
{
    const E: Self;
    const FRAC_1_PI: Self;
    const FRAC_1_SQRT_2: Self;
    const FRAC_2_PI: Self;
    const FRAC_2_SQRT_PI: Self;
    const FRAC_PI_2: Self;
    const FRAC_PI_3: Self;
    const FRAC_PI_4: Self;
    const FRAC_PI_6: Self;
    const FRAC_PI_8: Self;
    const LN_10: Self;
    const LN_2: Self;
    const LOG10_2: Self;
    const LOG10_E: Self;
    const LOG2_10: Self;
    const LOG2_E: Self;
    const PI: Self;
    const SQRT_2: Self;
    const TAU: Self;

    fn powi(self, n: i32) -> Self;
    fn sqrt(self) -> Self;
    fn exp(self) -> Self;
    fn abs(self) -> Self;
    fn atan(self) -> Self;
    fn sin(self) -> Self;
    fn cos(self) -> Self;
    fn tan(self) -> Self;
}

#[cfg(test)]
mod tests {

    /*
    #[test]
    fn vec2_floats() {
        let f = Float(0.5);
        let vec = Vec2 {
            x: Float(1.0),
            y: Float(2.23),
        };

        println!("vec + f = {}", vec.add(f));
        println!("f + vec = {}", f.add(vec));
    }

    #[test]
    fn vec2_intervals() {
        let f = Interval(0.5, 0.6);
        let vec = Vec2 {
            x: Interval(1.0, 1.1),
            y: Interval(2.23, 2.25),
        };

        println!("vec + f = {}", vec.add(f));
        println!("f + vec = {}", f.add(vec));
    }
     */
}
