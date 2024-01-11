mod float;
mod interval;
//mod point2;
mod vec2;

//use std::ops::{Add, Div, Mul, Neg, Sub};

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
    //const ZERO: Self;
    //const ONE: Self;

    fn powi(self, n: i32) -> Self;
    fn sqrt(self) -> Self;
    //
}

pub trait SArithmetic {
    fn add(self, rhs: Self) -> Self;
    fn sub(self, rhs: Self) -> Self;
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
