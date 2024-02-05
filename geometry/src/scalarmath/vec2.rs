use std::ops::Add;

use super::Scalar;

pub fn vec2<S: Scalar>(x: S, y: S) -> Vec2<S> {
    Vec2::new(x, y)
}

#[derive(Copy, Clone)]
pub struct Vec2<S: Scalar> {
    pub x: S,
    pub y: S,
}
impl<S: Scalar> Vec2<S> {
    pub fn new(x: S, y: S) -> Self {
        Self { x, y }
    }

    pub fn dot(self, rhs: Self) -> S {
        self.x * rhs.x + self.y * rhs.y
    }
}
/*
impl<S: Scalar> SAdd<S> for Vec2<S> {
    type Output = Vec2<S>;

    fn add(self, rhs: S) -> Self::Output {
        Vec2 {
            x: self.x.add(rhs),
            y: self.y.add(rhs),
        }
    }
}
 */
impl<S: Scalar> std::fmt::Display for Vec2<S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("[{}, {}]", self.x, self.y))
    }
}

/*
impl<S: Scalar> SAdd<Vec2<S>> for S {
    type Output = Vec2<S>;

    fn add(self, rhs: Vec2<S>) -> Self::Output {
        Vec2 {
            x: self.add(rhs.x),
            y: self.add(rhs.y),
        }
    }
}
 */

impl<S: Scalar> Add<S> for Vec2<S> {
    type Output = Vec2<S>;

    fn add(self, rhs: S) -> Self::Output {
        vec2(self.x + rhs, self.y + rhs)
    }
}

impl<S: Scalar> Add<Vec2<S>> for Vec2<S> {
    type Output = Vec2<S>;

    fn add(self, rhs: Vec2<S>) -> Self::Output {
        vec2(self.x + rhs.x, self.y + rhs.y)
    }
}

#[cfg(test)]
mod tests {
    use crate::scalarmath::{vec2, Float, Interval};

    #[test]
    fn vec2_dot() {
        println!(
            "{}",
            vec2(
                Interval(Float(7.0), Float(7.1)),
                Interval(Float(1.95), Float(2.05)),
            )
            .dot(vec2(
                Interval(Float(3.01), Float(3.011)),
                Interval(Float(5.99), Float(6.01)),
            ))
        );
    }

    #[test]
    fn vec2_add_scalar() {
        println!(
            "{}",
            vec2(
                Interval(Float(7.0), Float(7.1)),
                Interval(Float(1.95), Float(2.05)),
            ) + Interval::thin(Float(1.0))
        );
    }
}
