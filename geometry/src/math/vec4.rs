use crate::math::Mat44;

use super::Scalar;
use gen_ops::gen_ops;

pub fn vec4<S: Scalar>(x: S, y: S, z: S, w: S) -> Vec4<S> {
    Vec4::new(x, y, z, w)
}

#[derive(Copy, Clone)]
pub struct Vec4<S: Scalar> {
    pub x: S,
    pub y: S,
    pub z: S,
    pub w: S,
}
impl<S: Scalar> Vec4<S> {
    pub const ONES: Self = Self {
        x: S::ONE,
        y: S::ONE,
        z: S::ONE,
        w: S::ONE,
    };

    pub const ZERO: Self = Self {
        x: S::ZERO,
        y: S::ZERO,
        z: S::ZERO,
        w: S::ZERO,
    };

    pub const UNIT_X: Self = Self {
        x: S::ONE,
        y: S::ZERO,
        z: S::ZERO,
        w: S::ZERO,
    };

    pub const UNIT_Y: Self = Self {
        x: S::ZERO,
        y: S::ONE,
        z: S::ZERO,
        w: S::ZERO,
    };

    pub const UNIT_Z: Self = Self {
        x: S::ZERO,
        y: S::ZERO,
        z: S::ONE,
        w: S::ZERO,
    };

    pub const UNIT_W: Self = Self {
        x: S::ZERO,
        y: S::ZERO,
        z: S::ZERO,
        w: S::ONE,
    };

    pub fn new(x: S, y: S, z: S, w: S) -> Self {
        Self { x, y, z, w }
    }

    pub fn magnitude2(self) -> S {
        self.dot(self)
    }

    pub fn magnitude(self) -> S {
        self.magnitude2().sqrt()
    }

    pub fn dot(self, other: Self) -> S {
        self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w
    }

    pub fn normalize(self) -> Self {
        let mag = self.magnitude();
        self / mag
    }
}
impl<S: Scalar> std::fmt::Display for Vec4<S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "[{}, {}, {}, {}]",
            self.x, self.y, self.z, self.w
        ))
    }
}
impl<S: Scalar> std::fmt::Debug for Vec4<S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "[{}, {}, {}, {}]",
            self.x, self.y, self.z, self.w
        ))
    }
}

gen_ops!(
    <S>;
    types Vec4<S> => Vec4<S>;
    for - call |v: &Vec4<S>| {
        vec4(-v.x, -v.y, -v.z, -v.w)
    };
    where S: Scalar
);

gen_ops!(
    <S>;
    types Vec4<S>, Vec4<S> => Vec4<S>;
    for + call |l: &Vec4<S>, r: &Vec4<S>| {
        vec4(l.x + r.x, l.y + r.y, l.z + r.z, l.w + r.w)
    };
    for - call |l: &Vec4<S>, r: &Vec4<S>| {
        vec4(l.x - r.x, l.y - r.y, l.z - r.z, l.w - r.w)
    };
    for * call |l: &Vec4<S>, r: &Vec4<S>| {
        vec4(l.x * r.x, l.y * r.y, l.z * r.z, l.w * r.w)
    };
    for / call |l: &Vec4<S>, r: &Vec4<S>| {
        vec4(l.x / r.x, l.y / r.y, l.z / r.z, l.w / r.w)
    };
    where S: Scalar
);

gen_ops!(
    <S>;
    types Vec4<S>, Mat44<S> => Vec4<S>;
    for * call |l: &Vec4<S>, r: &Mat44<S>| {
        vec4(
            l.x * r[0][0] + l.y * r[1][0] + l.z * r[2][0] + l.w * r[3][0],
            l.x * r[0][1] + l.y * r[1][1] + l.z * r[2][1] + l.w * r[3][1],
            l.x * r[0][2] + l.y * r[1][2] + l.z * r[2][2] + l.w * r[3][2],
            l.x * r[0][3] + l.y * r[1][3] + l.z * r[2][3] + l.w * r[3][3],
        )
    };
    where S: Scalar
);

gen_ops!(
    <S>;
    types Mat44<S>, Vec4<S> => Vec4<S>;
    for * call |l: &Mat44<S>, r: &Vec4<S>| {
        vec4(
            l[0][0] * r.x + l[0][1] * r.y + l[0][2] * r.z + l[0][3] * r.w,
            l[1][0] * r.x + l[1][1] * r.y + l[1][2] * r.z + l[1][3] + r.w,
            l[2][0] * r.x + l[2][1] * r.y + l[2][2] * r.z + l[2][3] + r.w,
            l[3][0] * r.x + l[3][1] * r.y + l[3][2] * r.z + l[3][3] + r.w,
        )
    };
    where S: Scalar
);

gen_ops!(
    <S>;
    types Vec4<S>, S => Vec4<S>;
    for + call |l: &Vec4<S>, r: &S| {
        vec4(l.x + *r, l.y + *r, l.z + *r, l.w + *r)
    };
    for - call |l: &Vec4<S>, r: &S| {
        vec4(l.x - *r, l.y - *r, l.z - *r, l.w - *r)
    };
    for * call |l: &Vec4<S>, r: &S| {
        vec4(l.x * *r, l.y * *r, l.z * *r, l.w * *r)
    };
    for / call |l: &Vec4<S>, r: &S| {
        vec4(l.x / *r, l.y / *r, l.z / *r, l.w / *r)
    };
    where S: Scalar
);

/*
impl_op_ex!(-|a: &Vec4| -> Vec4 { vec4(-a.x, -a.y, -a.z, -a.w,) });

impl_op_ex_commutative!(*|v: &Vec4, s: f64| -> Vec4 { vec4(v.x * s, v.y * s, v.z * s, v.w * s,) });
impl_op_ex!(/|v: &Vec4, s: f64| -> Vec4 { vec4(v.x / s, v.y / s, v.z / s, v.w / s,) });
impl_op_ex!(/|s: f64, v: &Vec4| -> Vec4 { vec4(s / v.x, s / v.y, s / v.z, s / v.w) });

impl_op_ex!(-|a: &Vec4, b: &Vec4| -> Vec4 { vec4(a.x - b.x, a.y - b.y, a.z - b.z, a.w - b.w,) });
impl_op_ex!(+|a: &Vec4, b: &Vec4| -> Vec4 { vec4(a.x + b.x, a.y + b.y, a.z + b.z, a.w + b.w,) });
impl_op_ex!(*|a: &Vec4, b: &Vec4| -> Vec4 { vec4(a.x * b.x, a.y * b.y, a.z * b.z, a.w * b.w,) });
impl_op_ex!(/|a: &Vec4, b: &Vec4| -> Vec4 { vec4(a.x / b.x, a.y / b.y, a.z / b.z, a.w / b.w,) });
 */
