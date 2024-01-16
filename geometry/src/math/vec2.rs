use super::{rad, Angle, Scalar};
use crate::math::Mat22;
use gen_ops::gen_ops;

pub fn vec2<S: Scalar>(x: S, y: S) -> Vec2<S> {
    Vec2::new(x, y)
}

pub fn vec2_f32s<S: Scalar>(x: f32, y: f32) -> Vec2<S> {
    Vec2::new(x.into(), y.into())
}

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

    pub fn all(s: S) -> Self {
        Self { x: s, y: s }
    }

    pub fn u(self) -> S {
        self.x
    }

    pub fn v(self) -> S {
        self.y
    }

    pub fn clamp(self, min: Self, max: Self) -> Self {
        Self {
            x: self.x.clamp(min.x, max.x),
            y: self.y.clamp(min.y, max.y),
        }
    }

    pub fn magnitude2(self) -> S {
        self.dot(self)
    }

    pub fn magnitude(self) -> S {
        self.magnitude2().sqrt()
    }

    pub fn dot(self, other: Self) -> S {
        self.x * other.x + self.y * other.y
    }

    pub fn normalize(self) -> Self {
        self / self.magnitude()
    }

    pub fn orthogonal(self) -> Self {
        Self {
            x: -self.y,
            y: self.x,
        }
    }

    pub fn to_f64s(self) -> [f64; 2] {
        [self.x.as_f64(), self.y.as_f64()]
    }

    pub fn to_f32s(self) -> [f32; 2] {
        [self.x.as_f32(), self.y.as_f32()]
    }

    pub fn lerp(self, other: Self, t: S) -> Self {
        (Self::ONES - t) * self + other * t
    }
}
impl<S: Scalar> From<[f64; 2]> for Vec2<S> {
    fn from(floats: [f64; 2]) -> Self {
        Self {
            x: S::exact(floats[0]),
            y: S::exact(floats[1]),
        }
    }
}
impl<S: Scalar> From<[f32; 2]> for Vec2<S> {
    fn from(floats: [f32; 2]) -> Self {
        Self {
            x: S::exact(floats[0] as f64),
            y: S::exact(floats[1] as f64),
        }
    }
}
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

gen_ops!(
    <S>;
    types Vec2<S> => Vec2<S>;
    for - call |v: &Vec2<S>| {
        vec2(-v.x, -v.y)
    };
    where S: Scalar
);

gen_ops!(
    <S>;
    types Vec2<S>, Vec2<S> => Vec2<S>;
    for + call |l: &Vec2<S>, r: &Vec2<S>| {
        vec2(l.x + r.x, l.y + r.y)
    };
    for - call |l: &Vec2<S>, r: &Vec2<S>| {
        vec2(l.x - r.x, l.y - r.y)
    };
    for * call |l: &Vec2<S>, r: &Vec2<S>| {
        vec2(l.x * r.x, l.y * r.y)
    };
    for / call |l: &Vec2<S>, r: &Vec2<S>| {
        vec2(l.x / r.x, l.y / r.y)
    };
    where S: Scalar
);

gen_ops!(
    <S>;
    types Vec2<S>, Mat22<S> => Vec2<S>;
    for * call |l: &Vec2<S>, r: &Mat22<S>| {
        vec2(
            l.x * r[0][0] + l.y * r[1][0],
            l.x * r[0][1] + l.y * r[1][1],
        )
    };
    where S: Scalar
);

gen_ops!(
    <S>;
    types Mat22<S>, Vec2<S> => Vec2<S>;
    for * call |l: &Mat22<S>, r: &Vec2<S>| {
        vec2(
            l[0][0] * r.x + l[0][1] * r.y,
            l[1][0] * r.x + l[1][1] * r.y,
        )
    };
    where S: Scalar
);

gen_ops!(
    <S>;
    types Vec2<S>, S => Vec2<S>;
    for + call |l: &Vec2<S>, r: &S| {
        vec2(l.x + *r, l.y + *r)
    };
    for - call |l: &Vec2<S>, r: &S| {
        vec2(l.x - *r, l.y - *r)
    };
    for * call |l: &Vec2<S>, r: &S| {
        vec2(l.x * *r, l.y * *r)
    };
    for / call |l: &Vec2<S>, r: &S| {
        vec2(l.x / *r, l.y / *r)
    };
    where S: Scalar
);

#[cfg(test)]
mod tests {
    use crate::math::{deg, Mat33};

    use super::*;

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

    /*
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
     */

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
}
