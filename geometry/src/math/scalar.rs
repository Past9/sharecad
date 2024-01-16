use std::ops::{Add, Div, Mul, Neg, Sub};

pub trait Scalar:
    std::fmt::Debug
    + std::fmt::Display
    + Clone
    + Copy
    + Neg<Output = Self>
    + Add<Self, Output = Self>
    + Sub<Self, Output = Self>
    + Mul<Self, Output = Self>
    + Div<Self, Output = Self>
    + From<f32>
{
    const ZERO: Self;
    const HALF: Self;
    const ONE: Self;
    const TWO: Self;

    fn as_f64(self) -> f64;
    fn as_f32(self) -> f32;
}

impl Scalar for f64 {
    const ZERO: Self = 0.0;
    const HALF: Self = 0.5;
    const ONE: Self = 1.0;
    const TWO: Self = 2.0;

    fn as_f64(self) -> f64 {
        self
    }

    fn as_f32(self) -> f32 {
        self as f32
    }
}
