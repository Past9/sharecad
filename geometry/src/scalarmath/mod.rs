mod float;
mod interval;
//mod point2;
mod vec2;

//use std::ops::{Add, Div, Mul, Neg, Sub};

use std::ops::Add;

use auto_impl::auto_impl;
use float_cmp::Ulps;

pub use float::*;
pub use interval::*;
//pub use point2::*;
pub use vec2::*;

pub trait Scalar
where
    Self: std::fmt::Display + Copy + Clone + SArithmetic,
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

pub trait SArithmetic {
    fn neg(self) -> Self;
    //fn s_add(self, rhs: Self) -> Self;
    //fn s_sub(self, rhs: Self) -> Self;
    fn mul(self, rhs: Self) -> Self;
    fn div(self, rhs: Self) -> Self;
    fn eq(self, rhs: Self) -> bool;
    fn neq(self, rhs: Self) -> bool;
    fn lt(self, rhs: Self) -> bool;
    fn lte(self, rhs: Self) -> bool;
    fn gt(self, rhs: Self) -> bool;
    fn gte(self, rhs: Self) -> bool;
}

#[cfg(test)]
mod tests {
    use crate::scalarmath::{Float, Interval, SArithmetic, Vec2};

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
