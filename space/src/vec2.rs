use crate::Point2;
use auto_ops::{impl_op_ex, impl_op_ex_commutative};
use std::borrow::Borrow;

pub fn vec2(x: f64, y: f64) -> Vec2 {
    Vec2::new(x, y)
}

#[derive(Copy, Clone)]
pub struct Vec2 {
    pub x: f64,
    pub y: f64,
}
impl Vec2 {
    pub const UNIT_X: Self = Self { x: 1.0, y: 0.0 };
    pub const UNIT_Y: Self = Self { x: 0.0, y: 1.0 };

    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    pub fn magnitude2(&self) -> f64 {
        self.x.powi(2) + self.y.powi(2)
    }

    pub fn magnitude(&self) -> f64 {
        self.magnitude2().sqrt()
    }

    pub fn dot<T: Borrow<Self>>(&self, other: T) -> f64 {
        let other = other.borrow();
        self.x * other.x + self.y * other.y
    }

    pub fn normalize(&self) -> Self {
        let mag = self.magnitude();
        Self {
            x: self.x / mag,
            y: self.y / mag,
        }
    }

    pub fn orthogonal(&self) -> Self {
        Self {
            x: self.y,
            y: -self.x,
        }
    }

    pub fn to_point(&self) -> Point2 {
        self.into()
    }
}
impl<T: Borrow<Point2>> From<T> for Vec2 {
    fn from(point: T) -> Self {
        let point = point.borrow();
        Self {
            x: point.x,
            y: point.y,
        }
    }
}
impl std::fmt::Display for Vec2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("[{}, {}]", self.x, self.y))
    }
}
impl std::fmt::Debug for Vec2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("[{}, {}]", self.x, self.y))
    }
}

impl_op_ex!(+|a: Vec2, b: Vec2| -> Vec2 { vec2(a.x + b.x, a.y + b.y) });
impl_op_ex!(-|a: Vec2, b: Vec2| -> Vec2 { vec2(a.x - b.x, a.y - b.y) });
impl_op_ex_commutative!(*|v: Vec2, s: f64| -> Vec2 { vec2(v.x * s, v.y * s) });
impl_op_ex_commutative!(/|v: Vec2, s: f64| -> Vec2 { vec2(v.x / s, v.y / s) });
