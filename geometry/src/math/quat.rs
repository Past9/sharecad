use auto_ops::impl_op_ex;
use gen_ops::gen_ops;

use super::{vec3, Angle, Mat33, Mat44, Scalar, Vec3};

#[derive(Debug, Clone, Copy)]
pub struct Quat<S: Scalar> {
    pub v: Vec3<S>,
    pub s: S,
}
impl<S: Scalar> Quat<S> {
    pub const ZERO: Self = Self {
        v: Vec3::ZERO,
        s: S::ONE,
    };

    pub fn new(w: S, xi: S, yj: S, zk: S) -> Self {
        Self::from_sv(w, vec3(xi, yj, zk))
    }

    pub fn from_sv(s: S, v: Vec3<S>) -> Self {
        Quat { s: s, v: v }
    }

    pub fn from_axis_angle(axis: Vec3<S>, angle: Angle<S>) -> Self {
        let (sin, cos) = (angle * S::HALF).sin_cos();
        Self::from_sv(cos, axis.normalize() * sin)
    }

    pub fn to_mat33(self) -> Mat33<S> {
        self.into()
    }

    pub fn to_mat44(self) -> Mat44<S> {
        self.into()
    }

    pub fn dot(self, other: Self) -> S {
        self.s * other.s + self.v.dot(other.v)
    }

    pub fn magnitude2(self) -> S {
        self.dot(self)
    }

    pub fn magnitude(self) -> S {
        self.magnitude2().sqrt()
    }

    pub fn normalize(self) -> Self {
        self / self.magnitude()
    }
}

gen_ops!(
    <S>;
    types Quat<S> => Quat<S>;
    for - call |q: &Quat<S>| {
        Quat::from_sv(-q.s, -q.v)
    };
    where S: Scalar
);

gen_ops!(
    <S>;
    types Quat<S>, Quat<S> => Quat<S>;
    for + call |l: &Quat<S>, r: &Quat<S>| {
        Quat::from_sv(l.s + r.s, l.v + r.v)
    };
    for - call |l: &Quat<S>, r: &Quat<S>| {
        Quat::from_sv(l.s - r.s, l.v - r.v)
    };
    for * call |l: &Quat<S>, r: &Quat<S>| {
        Quat::from_sv(l.s * r.s - l.v.dot(r.v), l.s * r.v + r.s * l.v + l.v.cross(r.v))
    };
    for / call |l: &Quat<S>, r: &Quat<S>| {
        Quat::from_sv(l.s / r.s, l.v / r.v)
    };
    where S: Scalar
);

gen_ops!(
    <S>;
    types Quat<S>, S => Quat<S>;
    for * call |l: &Quat<S>, r: &S| {
        Quat::from_sv(l.s * *r, l.v * *r)
    };
    for / call |l: &Quat<S>, r: &S| {
        Quat::from_sv(l.s / *r, l.v / *r)
    };
    where S: Scalar
);

gen_ops!(
    <S>;
    types Quat<S>, Vec3<S> => Vec3<S>;
    for * call |q: &Quat<S>, v: &Vec3<S>| {
        let tmp = q.v.cross(*v) + (*v * q.s);
        (q.v.cross(tmp) * S::TWO) + *v
    };
    where S: Scalar
);

/*
// Unary
impl_op_ex!(-|q: Quat| -> Quat { Quat::from_sv(-q.s, -q.v) });

// Binary non-commutative
impl_op_ex!(*|q: Quat, s: f64| -> Quat { Quat::from_sv(q.s * s, q.v * s) });
impl_op_ex!(/|q: Quat, s: f64| -> Quat { Quat::from_sv(q.s / s, q.v / s) });
impl_op_ex!(-|a: Quat, b: Quat| -> Quat { Quat::from_sv(a.s - b.s, a.v - b.v) });
impl_op_ex!(*|a: Quat, b: Quat| -> Quat {
    Quat::new(
        a.s * b.s - a.v.x * b.v.x - a.v.y * b.v.y - a.v.z * b.v.z,
        a.s * b.v.x + a.v.x * b.s + a.v.y * b.v.z - a.v.z * b.v.y,
        a.s * b.v.y + a.v.y * b.s + a.v.z * b.v.x - a.v.x * b.v.z,
        a.s * b.v.z + a.v.z * b.s + a.v.x * b.v.y - a.v.y * b.v.x,
    )
});
impl_op_ex!(*|q: Quat, v: Vec3| -> Vec3 {
    let tmp = q.v.cross(v) + (v * q.s);
    (q.v.cross(tmp) * 2.0) + v
});
impl_op_ex!(*|q: Quat, p: Point3| -> Point3 { (q * p.into_vec()).into_point() });

// Binary commutative
impl_op_ex!(+|a: Quat, b: Quat| -> Quat { Quat::from_sv(a.s + b.s, a.v + b.v) });

// Assignment
impl_op_ex!(+= |a: &mut Quat, b: Quat| {
    a.s += b.s;
    a.v += b.v;
});
impl_op_ex!(*= |a: &mut Quat, b: Quat| {
    let product = *a * b;
    a.s = product.s;
    a.v = product.v;
});
impl_op_ex!(-= |a: &mut Quat, b: Quat| {
    a.s -= b.s;
    a.v -= b.v;
});
 */

/*
impl_operator!(<S: BaseFloat> Mul<S> for Quaternion<S> {
    fn mul(lhs, rhs) -> Quaternion<S> {
        Quaternion::from_sv(lhs.s * rhs, lhs.v * rhs)
    }
});
 */
