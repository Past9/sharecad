use auto_ops::{impl_op_ex, impl_op_ex_commutative};

use super::{vec2, Mat33, Scalar, Vec2};

pub fn point2s<S: Scalar>(x: S, y: S) -> Point2<S> {
    Point2::new(x, y)
}

/*
pub fn point2_f32s(x: f32, y: f32) -> Point2s {
    Point2s::new(x as f64, y as f64)
}
 */

#[derive(Copy, Clone)]
pub struct Point2<S: Scalar> {
    pub x: S,
    pub y: S,
}
impl<S: Scalar> Point2<S> {
    pub const ZERO: Self = Point2 {
        x: S::ZERO,
        y: S::ZERO,
    };

    pub fn new(x: S, y: S) -> Self {
        Self { x, y }
    }

    pub fn u(&self) -> S {
        self.x
    }

    pub fn v(&self) -> S {
        self.y
    }

    /*
    pub fn into_vec(&self) -> Vec2 {
        (*self).into()
    }
     */

    /*
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
      */
}
/*
impl From<Vec2> for Point2 {
    fn from(vec: Vec2) -> Self {
        Self { x: vec.x, y: vec.y }
    }
}
 */
impl<S: Scalar> std::fmt::Display for Point2<S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("({}, {})", self.x, self.y))
    }
}
impl<S: Scalar> std::fmt::Debug for Point2<S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("({}, {})", self.x, self.y))
    }
}

/*
impl_op_ex_commutative!(+|p: Vec2, v: Point2| -> Point2 {
    point2(p.x + v.x, p.y + v.y)
});
impl_op_ex!(-|p: Point2, v: Vec2| -> Point2 { point2(p.x - v.x, p.y - v.y) });
 */
//impl_op_ex!(-|a: Point2s, b: Point2s| -> Vec2 { vec2(a.x - b.x, a.y - b.y) });
