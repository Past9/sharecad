mod float;
//mod point2;
//mod vec2;

use std::ops::{Add, Div, Mul, Neg, Sub};

use auto_impl::auto_impl;

pub use float::*;
//pub use point2::*;
//pub use vec2::*;

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
    //const ZERO: Self;
    //const ONE: Self;

    fn clamp(self, min: Self, max: Self) -> Self;
    fn powi(self, n: i32) -> Self;
    fn sqrt(self) -> Self;
    //
}

/*
struct Vec2<S: Scalar> {
    x: S,
    y: S,
}
 */
