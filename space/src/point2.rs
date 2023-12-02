use crate::{vec2, Mat33, Vec2};
use auto_ops::{impl_op_ex, impl_op_ex_commutative};

pub fn point2(x: f64, y: f64) -> Point2 {
    Point2::new(x, y)
}

pub fn point2_f32s(x: f32, y: f32) -> Point2 {
    Point2::new(x as f64, y as f64)
}

#[derive(Copy, Clone)]
pub struct Point2 {
    pub x: f64,
    pub y: f64,
}
impl Point2 {
    pub const ZERO: Self = Point2 { x: 0.0, y: 0.0 };

    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    pub fn into_vec(&self) -> Vec2 {
        (*self).into()
    }

    pub fn transform(&self, m: Mat33) -> Self {
        let x = (self.x * m[0][0]) + (self.y * m[0][1]) + m[0][2];
        let y = (self.x * m[1][0]) + (self.y * m[1][1]) + m[1][2];
        let z = (self.x * m[2][0]) + (self.y * m[2][1]) + m[2][2];
        Self { x: x / z, y: y / z }
    }

    pub fn to_f64s(&self) -> [f64; 2] {
        [self.x, self.y]
    }

    pub fn to_f32s(&self) -> [f32; 2] {
        [self.x as f32, self.y as f32]
    }
}
impl From<Vec2> for Point2 {
    fn from(vec: Vec2) -> Self {
        Self { x: vec.x, y: vec.y }
    }
}
impl std::fmt::Display for Point2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("({}, {})", self.x, self.y))
    }
}
impl std::fmt::Debug for Point2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("({}, {})", self.x, self.y))
    }
}

impl_op_ex_commutative!(+|p: Vec2, v: Point2| -> Point2 {
    point2(p.x + v.x, p.y + v.y)
});
impl_op_ex!(-|p: Point2, v: Vec2| -> Point2 { point2(p.x - v.x, p.y - v.y) });
impl_op_ex!(-|a: Point2, b: Point2| -> Vec2 { vec2(a.x - b.x, a.y - b.y) });
