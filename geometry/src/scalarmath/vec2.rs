use std::ops::{Add, Div, Mul, Neg, Sub};

use auto_ops::{impl_op_ex, impl_op_ex_commutative};

use super::{Point2, Scalar};

pub fn vec2<S: Scalar>(x: S, y: S) -> Vec2<S> {
    Vec2::new(x, y)
}

/*
pub fn vec2_f32s(x: f32, y: f32) -> Vec2 {
    Vec2::new(x as f64, y as f64)
}
 */

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum TurnDir {
    Cw,
    Ccw,
    Aligned,
    Opposite,
}

#[derive(Copy, Clone, PartialEq)]
pub struct Vec2<S: Scalar> {
    pub x: S,
    pub y: S,
}
impl<S: Scalar> Vec2<S> {
    pub const ONES: Self = Self {
        x: S::ONE,
        y: S::ONE,
    };
    pub const ZERO: Self = Self {
        x: S::ZERO,
        y: S::ZERO,
    };
    pub const UNIT_X: Self = Self {
        x: S::ONE,
        y: S::ZERO,
    };
    pub const UNIT_Y: Self = Self {
        x: S::ZERO,
        y: S::ONE,
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

    pub fn clamp(&self, min: Self, max: Self) -> Self {
        Self {
            x: self.x.clamp(min.x, max.x),
            y: self.y.clamp(min.y, max.y),
        }
    }

    /*
    pub fn angle(&self) -> Angle {
        rad(self.y.atan2(self.x))
    }
     */

    pub fn magnitude2(&self) -> S {
        self.x.powi(2) + self.y.powi(2)
    }

    pub fn magnitude(&self) -> S {
        self.magnitude2().sqrt()
    }

    pub fn dot(&self, other: Vec2<S>) -> S {
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
            x: -self.y,
            y: self.x,
        }
    }

    pub fn into_point(&self) -> Point2<S> {
        (*self).into()
    }

    /*
    pub fn to_f64s(&self) -> [f64; 2] {
        [self.x, self.y]
    }

    pub fn to_f32s(&self) -> [f32; 2] {
        [self.x as f32, self.y as f32]
    }
     */

    pub fn lerp(&self, other: Self, t: S) -> Self {
        let x = self - t;
        (Self::ONES - t) * self + t * other
    }
}

impl<S: Scalar> Add<Vec2<S>> for Vec2<S> {
    type Output = Self;

    fn add(self, rhs: Vec2<S>) -> Self::Output {
        todo!()
    }
}

impl<S: Scalar> Add<S> for Vec2<S> {
    type Output = Self;

    fn add(self, rhs: S) -> Self::Output {
        todo!()
    }
}

impl<S: Scalar> Add<Vec2<S>> for S {
    type Output = Vec2<S>;

    fn add(self, rhs: Vec2<S>) -> Self::Output {
        todo!()
    }
}

/*
impl_op_ex!(+ <S: Scalar> |l: &Vec2<S>, r: &Vec2<S>| -> Vec2<S> {
    Vec2 {
        x: l.x + r.x,
        y: l.y + r.y
    }
});
impl_op_ex!(- <S: Scalar> |l: &Vec2<S>, r: &Vec2<S>| -> Vec2<S> {
    Vec2 {
        x: l.x - r.x,
        y: l.y - r.y
    }
});
impl_op_ex!(* <S: Scalar> |l: &Vec2<S>, r: &Vec2<S>| -> Vec2<S> {
    Vec2 {
        x: l.x * r.x,
        y: l.y * r.y
    }
});
impl_op_ex!(/ <S: Scalar> |l: &Vec2<S>, r: &Vec2<S>| -> Vec2<S> {
    Vec2 {
        x: l.x / r.x,
        y: l.y / r.y
    }
});

impl_op_ex!(- <S: Scalar> |l: &Vec2<S>, r: S| -> Vec2<S> {
    Vec2 {
        x: l.x - r,
        y: l.y - r
    }
});
*/

/*
impl_op_ex_commutative!(- <S: Scalar> |l: &Vec2<S>, r: S| -> Vec2<S> {
    Vec2 {
        x: l.x - r,
        y: l.y - r
    }
});
impl_op_ex_commutative!(+ <S: Scalar> |l: &Vec2<S>, r: S| -> Vec2<S> {
    Vec2 {
        x: l.x + r,
        y: l.y + r
    }
});
impl_op_ex_commutative!(* <S: Scalar> |l: &Vec2<S>, r: S| -> Vec2<S> {
    Vec2 {
        x: l.x * r,
        y: l.y * r
    }
});
impl_op_ex_commutative!(/ <S: Scalar> |l: &Vec2<S>, r: S| -> Vec2<S> {
    Vec2 {
        x: l.x / r,
        y: l.y / r
    }
});
 */

/*
#[opimps::impl_op(Add)]
fn add<S: Scalar>(self: Vec2<S>, rhs: S) -> Vec2<S> {
    Vec2 {
        x: self.x + rhs,
        y: self.y + rhs,
    }
}
#[opimps::impl_op(Sub)]
fn sub<S: Scalar>(self: Vec2<S>, rhs: S) -> Vec2<S> {
    Vec2 {
        x: self.x - rhs,
        y: self.y - rhs,
    }
}
#[opimps::impl_op(Mul)]
fn mul<S: Scalar>(self: Vec2<S>, rhs: S) -> Vec2<S> {
    Vec2 {
        x: self.x * rhs,
        y: self.y * rhs,
    }
}
 */
/*
#[opimps::impl_op(Sub)]
fn sub<S: Scalar>(self: &Vec2<S>, rhs: &S) -> Vec2<S> {
    Vec2 {
        x: self.x + *rhs,
        y: self.y + *rhs,
    }
}
#[opimps::impl_op(Sub)]
fn sub<S: Scalar>(self: Vec2<S>, rhs: &S) -> Vec2<S> {
    Vec2 {
        x: self.x + *rhs,
        y: self.y + *rhs,
    }
}
#[opimps::impl_op(Sub)]
fn sub<S: Scalar>(self: &Vec2<S>, rhs: S) -> Vec2<S> {
    Vec2 {
        x: self.x + rhs,
        y: self.y + rhs,
    }
}
 */

/*
impl<S: Scalar> Neg for Vec2<S> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }
}
impl<S: Scalar> Add<S> for Vec2<S> {
    type Output = Self;

    fn add(self, rhs: S) -> Self::Output {
        Self {
            x: self.x + rhs,
            y: self.y + rhs,
        }
    }
}
impl<S: Scalar> Add<Vec2<S>> for S {
    type Output = Vec2<S>;

    fn add(self, rhs: Vec2<S>) -> Self::Output {
        Vec2 {
            x: self + rhs.x,
            y: self + rhs.y,
        }
    }
}
impl<S: Scalar> Sub<S> for Vec2<S> {
    type Output = Self;

    fn sub(self, rhs: S) -> Self::Output {
        Self {
            x: self.x - rhs,
            y: self.y - rhs,
        }
    }
}
impl<S: Scalar> Mul<S> for Vec2<S> {
    type Output = Self;

    fn mul(self, rhs: S) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}
impl<S: Scalar> Div<S> for Vec2<S> {
    type Output = Self;

    fn div(self, rhs: S) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}
impl<S: Scalar> From<Point2<S>> for Vec2<S> {
    fn from(point: Point2<S>) -> Self {
        Self {
            x: point.x,
            y: point.y,
        }
    }
}
*/
/*
impl From<[f64; 2]> for Vec2 {
    fn from(floats: [f64; 2]) -> Self {
        Self {
            x: floats[0],
            y: floats[1],
        }
    }
}
impl From<[f32; 2]> for Vec2 {
    fn from(floats: [f32; 2]) -> Self {
        Self {
            x: floats[0] as f64,
            y: floats[1] as f64,
        }
    }
}
 */
impl<S: Scalar> std::fmt::Display for Vec2<S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("[{}, {}]", self.x, self.y))
    }
}
impl<S: Scalar> std::fmt::Debug for Vec2<S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("[{}, {}]", self.x, self.y))
    }
}

/*
impl_op_ex!(-|a: &Vec2| -> Vec2<S> { vec2(-a.x, -a.y) });
impl_op_ex!(+|a: &Vec2, b: &Vec2| -> Vec2 { vec2(a.x + b.x, a.y + b.y) });
impl_op_ex!(-|a: &Vec2, b: &Vec2| -> Vec2 { vec2(a.x - b.x, a.y - b.y) });
impl_op_ex!(*|a: &Vec2, b: &Vec2| -> Vec2 { vec2(a.x * b.x, a.y * b.y) });
impl_op_ex!(/|a: &Vec2, b: &Vec2| -> Vec2 { vec2(a.x / b.x, a.y / b.y) });
impl_op_ex_commutative!(*|v: &Vec2, s: &f64| -> Vec2 { vec2(v.x * s, v.y * s) });
impl_op_ex!(-|v: &Vec2, s: &f64| -> Vec2 { vec2(v.x - s, v.y - s) });
impl_op_ex!(/|v: &Vec2, s: &f64| -> Vec2 { vec2(v.x / s, v.y / s) });
impl_op_ex!(/|s: &f64, v: &Vec2| -> Vec2 { vec2(s / v.x, s / v.y) });
 */

#[cfg(test)]
mod tests {
    use crate::math::{deg, point2, Mat33};

    use super::*;

    /*
    #[test]
    fn gets_magnitude2() {
        assert_eq!(2.0, vec2(1.0, 1.0).magnitude2());
        assert_eq!(2.0, vec2(-1.0, 1.0).magnitude2());
        assert_eq!(2.0, vec2(1.0, -1.0).magnitude2());
        assert_eq!(2.0, vec2(-1.0, -1.0).magnitude2());
        assert_eq!(1.0, vec2(0.0, 1.0).magnitude2());
        assert_eq!(1.0, vec2(0.0, -1.0).magnitude2());
        assert_eq!(1.0, vec2(1.0, 0.0).magnitude2());
        assert_eq!(1.0, vec2(-1.0, 0.0).magnitude2());
    }

    #[test]
    fn gets_magnitude() {
        assert_cc!(2f64.sqrt(), vec2(1.0, 1.0).magnitude());
        assert_cc!(2f64.sqrt(), vec2(-1.0, 1.0).magnitude());
        assert_cc!(2f64.sqrt(), vec2(1.0, -1.0).magnitude());
        assert_cc!(2f64.sqrt(), vec2(-1.0, -1.0).magnitude());
        assert_cc!(1.0, vec2(0.0, 1.0).magnitude());
        assert_cc!(1.0, vec2(0.0, -1.0).magnitude());
        assert_cc!(1.0, vec2(1.0, 0.0).magnitude());
        assert_cc!(1.0, vec2(-1.0, 0.0).magnitude());
    }

    #[test]
    fn gets_dot() {
        // orthogonal
        assert_cc!(0.0, vec2(0.0, 1.0).dot(vec2(1.0, 0.0)));
        assert_cc!(0.0, vec2(0.0, -1.0).dot(vec2(1.0, 0.0)));
        assert_cc!(0.0, vec2(0.0, 1.0).dot(vec2(-1.0, 0.0)));
        assert_cc!(0.0, vec2(1.0, 0.0,).dot(vec2(0.0, 1.0)));
        assert_cc!(0.0, vec2(-1.0, 0.0,).dot(vec2(0.0, 1.0)));
        assert_cc!(0.0, vec2(1.0, 0.0,).dot(vec2(0.0, -1.0)));

        // equal
        assert_cc!(1.0, vec2(1.0, 0.0,).dot(vec2(1.0, 0.0)));
        assert_cc!(1.0, vec2(-1.0, 0.0,).dot(vec2(-1.0, 0.0)));
        assert_cc!(1.0, vec2(0.0, 1.0).dot(vec2(0.0, 1.0)));
        assert_cc!(1.0, vec2(0.0, -1.0).dot(vec2(0.0, -1.0)));

        // opposite
        assert_cc!(-1.0, vec2(1.0, 0.0,).dot(vec2(-1.0, 0.0)));
        assert_cc!(-1.0, vec2(-1.0, 0.0,).dot(vec2(1.0, 0.0)));
        assert_cc!(-1.0, vec2(0.0, 1.0).dot(vec2(0.0, -1.0)));
        assert_cc!(-1.0, vec2(0.0, -1.0).dot(vec2(0.0, 1.0)));

        assert_cc!(
            1.0,
            vec2(1.0, 1.0).normalize().dot(vec2(1.0, 1.0).normalize())
        );
    }

    #[test]
    fn normalizes_vec() {
        assert_cc!(1.0, vec2(1.0, 0.0).normalize().magnitude());
        assert_cc!(1.0, vec2(0.0, 1.0).normalize().magnitude());
        assert_cc!(1.0, vec2(1.0, 1.0).normalize().magnitude());

        assert_cc!(1.0, vec2(-1.0, 0.0).normalize().magnitude());
        assert_cc!(1.0, vec2(0.0, -1.0).normalize().magnitude());
        assert_cc!(1.0, vec2(-1.0, -1.0).normalize().magnitude());

        assert_cc!(1.0, vec2(1.0, -1.0).normalize().magnitude());
        assert_cc!(1.0, vec2(-1.0, 1.0).normalize().magnitude());

        assert_cc!(1.0, vec2(3.0, -7.0).normalize().magnitude());
        assert_cc!(1.0, vec2(-100.23, 3.426).normalize().magnitude());
    }

    #[test]
    fn gets_orthogonal() {
        assert_cc!(vec2(-2.0, -4.0), vec2(-4.0, 2.0).orthogonal());
        assert_cc!(vec2(-4.0, -2.0), vec2(-2.0, 4.0).orthogonal());
        assert_cc!(vec2(2.0, -4.0), vec2(-4.0, -2.0).orthogonal());
        assert_cc!(vec2(4.0, 2.0), vec2(2.0, -4.0).orthogonal());

        // Check that the orthogonal is equivalent to rotating the base
        // vector 90°
        let base_vec = vec2(4.0, 2.0);
        assert_cc!(
            base_vec.orthogonal().into_point(),
            base_vec.into_point().transform(Mat33::rotation(deg(90.0)))
        );
    }

    #[test]
    fn mul_vecs() {
        assert_cc!(vec2(-15.0, 77.0), vec2(-3.0, -11.0) * vec2(5.0, -7.0));
    }

    #[test]
    fn div_vecs() {
        assert_cc!(vec2(-0.6, 2.0), vec2(-3.0, -14.0) / vec2(5.0, -7.0));
    }

    #[test]
    fn add_vecs() {
        assert_cc!(vec2(2.0, -21.0), vec2(-3.0, -14.0) + vec2(5.0, -7.0));
    }

    #[test]
    fn sub_vecs() {
        assert_cc!(vec2(-8.0, -7.0), vec2(-3.0, -14.0) - vec2(5.0, -7.0));
    }

    #[test]
    fn vec_to_point() {
        assert_cc!(point2(-3.0, 14.0), vec2(-3.0, 14.0).into_point());
    }
    */
}
