use std::ops::{Add, Div, Mul, Neg, Sub};

use super::Coincidence;

pub const COINCIDENT_TOL: f64 = 1e-10;

pub trait Scalar:
    std::fmt::Debug
    + std::fmt::Display
    + PartialEq
    + Clone
    + Copy
    + Neg<Output = Self>
    + Add<Self, Output = Self>
    + Sub<Self, Output = Self>
    + Mul<Self, Output = Self>
    + Div<Self, Output = Self>
    + From<f32>
    + Coincidence<Self>
{
    const ZERO: Self;
    const HALF: Self;
    const ONE: Self;
    const TWO: Self;

    const PI: Self;
    const TAU: Self;
    const FRAC_PI_2: Self;
    const FRAC_PI_3: Self;
    const FRAC_PI_4: Self;
    const FRAC_PI_6: Self;
    const FRAC_PI_8: Self;
    const FRAC_1_PI: Self;
    const FRAC_2_PI: Self;
    const FRAC_2_SQRT_PI: Self;
    const SQRT_2: Self;
    const FRAC_1_SQRT_2: Self;
    const E: Self;
    const LOG2_10: Self;
    const LOG2_E: Self;
    const LOG10_2: Self;
    const LOG10_E: Self;
    const LN_2: Self;
    const LN_10: Self;

    fn as_f64(self) -> f64;
    fn as_f32(self) -> f32;
    fn exact(val: f64) -> Self;

    fn sin(self) -> Self;
    fn cos(self) -> Self;
    fn tan(self) -> Self;
    fn csc(self) -> Self;
    fn sec(self) -> Self;
    fn cot(self) -> Self;
    fn sin_cos(self) -> (Self, Self);

    fn recip(self) -> Self;
    fn abs(self) -> Self;
    fn powi(self, n: i32) -> Self;
    fn sqrt(self) -> Self;
    fn clamp(self, min: Self, max: Self) -> Self;
}

impl Scalar for f64 {
    const ZERO: Self = 0.0;
    const HALF: Self = 0.5;
    const ONE: Self = 1.0;
    const TWO: Self = 2.0;

    const PI: Self = std::f64::consts::PI;
    const TAU: Self = std::f64::consts::TAU;
    const FRAC_PI_2: Self = std::f64::consts::FRAC_PI_2;
    const FRAC_PI_3: Self = std::f64::consts::FRAC_PI_3;
    const FRAC_PI_4: Self = std::f64::consts::FRAC_PI_4;
    const FRAC_PI_6: Self = std::f64::consts::FRAC_PI_6;
    const FRAC_PI_8: Self = std::f64::consts::FRAC_PI_8;
    const FRAC_1_PI: Self = std::f64::consts::FRAC_1_PI;
    const FRAC_2_PI: Self = std::f64::consts::FRAC_2_PI;
    const FRAC_2_SQRT_PI: Self = std::f64::consts::FRAC_2_SQRT_PI;
    const SQRT_2: Self = std::f64::consts::SQRT_2;
    const FRAC_1_SQRT_2: Self = std::f64::consts::FRAC_1_SQRT_2;
    const E: Self = std::f64::consts::E;
    const LOG2_10: Self = std::f64::consts::LOG2_10;
    const LOG2_E: Self = std::f64::consts::LOG2_E;
    const LOG10_2: Self = std::f64::consts::LOG10_2;
    const LOG10_E: Self = std::f64::consts::LOG10_E;
    const LN_2: Self = std::f64::consts::LN_2;
    const LN_10: Self = std::f64::consts::LN_10;

    fn as_f64(self) -> f64 {
        self
    }

    fn as_f32(self) -> f32 {
        self as f32
    }

    fn exact(val: f64) -> Self {
        val
    }

    fn sin(self) -> Self {
        self.sin()
    }

    fn cos(self) -> Self {
        self.cos()
    }

    fn tan(self) -> Self {
        self.tan()
    }

    fn csc(self) -> Self {
        self.csc()
    }

    fn sec(self) -> Self {
        self.sec()
    }

    fn cot(self) -> Self {
        self.cot()
    }

    fn sin_cos(self) -> (Self, Self) {
        self.sin_cos()
    }

    fn recip(self) -> Self {
        1.0 / self
    }

    fn abs(self) -> Self {
        self.abs()
    }

    fn powi(self, n: i32) -> Self {
        self.powi(n)
    }

    fn sqrt(self) -> Self {
        self.sqrt()
    }

    fn clamp(self, min: Self, max: Self) -> Self {
        self.clamp(min, max)
    }
}
