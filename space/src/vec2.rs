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
    pub const ZERO: Self = Self { x: 0.0, y: 0.0 };
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
            x: -self.y,
            y: self.x,
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
impl_op_ex!(*|a: Vec2, b: Vec2| -> Vec2 { vec2(a.x * b.x, a.y * b.y) });
impl_op_ex!(/|a: Vec2, b: Vec2| -> Vec2 { vec2(a.x / b.x, a.y / b.y) });
impl_op_ex_commutative!(*|v: Vec2, s: f64| -> Vec2 { vec2(v.x * s, v.y * s) });
impl_op_ex_commutative!(/|v: Vec2, s: f64| -> Vec2 { vec2(v.x / s, v.y / s) });

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{deg, point2, Mat33};

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
        assert_cc!(vec2(-2.0, 4.0), vec2(4.0, 2.0).orthogonal());
        assert_cc!(vec2(-4.0, -2.0), vec2(-2.0, 4.0).orthogonal());
        assert_cc!(vec2(2.0, -4.0), vec2(-4.0, -2.0).orthogonal());
        assert_cc!(vec2(4.0, 2.0), vec2(2.0, -4.0).orthogonal());

        // Check that the orthogonal is equivalent to rotating the base
        // vector +90°
        let base_vec = vec2(4.0, 2.0);
        assert_cc!(
            base_vec.orthogonal().to_point(),
            base_vec.to_point().transform(Mat33::rotation(deg(90.0)))
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
        assert_cc!(point2(-3.0, 14.0), vec2(-3.0, 14.0).to_point());
    }
}
