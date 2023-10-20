use auto_ops::{impl_op, impl_op_ex};

use crate::{vec3, Angle, Mat33, Mat44, Point3, Vec3};

#[derive(Debug, Clone, Copy)]
pub struct Quat {
    pub v: Vec3,
    pub s: f64,
}
impl Quat {
    pub fn new(w: f64, xi: f64, yj: f64, zk: f64) -> Self {
        Self::from_sv(w, vec3(xi, yj, zk))
    }

    pub fn from_sv(s: f64, v: Vec3) -> Self {
        Quat { s: s, v: v }
    }

    pub fn from_axis_angle(axis: Vec3, angle: Angle) -> Self {
        let (sin, cos) = (angle * 0.5).sin_cos();
        Self::from_sv(cos, axis * sin)
    }

    pub fn to_mat33(&self) -> Mat33 {
        (*self).into()
    }

    pub fn to_mat44(&self) -> Mat44 {
        (*self).into()
    }
}

// Unary
impl_op_ex!(-|q: Quat| -> Quat { Quat::from_sv(-q.s, -q.v) });

// Binary non-commutative
impl_op_ex!(*|q: Quat, s: f64| -> Quat { Quat::from_sv(q.s * s, q.v * s) });
impl_op_ex!(/|q: Quat, s: f64| -> Quat { Quat::from_sv(q.s / s, q.v / s) });
impl_op_ex!(+|a: Quat, b: Quat| -> Quat { Quat::from_sv(a.s + b.s, a.v + b.v) });
impl_op_ex!(-|a: Quat, b: Quat| -> Quat { Quat::from_sv(a.s - b.s, a.v - b.v) });
impl_op_ex!(*|q: Quat, v: Vec3| -> Vec3 {
    let tmp = q.v.cross(v) + (v * q.s);
    (q.v.cross(tmp) * 2.0) + v
});
impl_op_ex!(*|q: Quat, p: Point3| -> Point3 { (q * p.into_vec()).into_point() });

// Assignment
impl_op_ex!(+= |a: &mut Quat, b: Quat| {
   a.s += b.s;
   a.v += b.v;
});
impl_op_ex!(-= |a: &mut Quat, b: Quat| {
   a.s -= b.s;
   a.v -= b.v;
});

/*
impl_operator!(<S: BaseFloat> Mul<S> for Quaternion<S> {
    fn mul(lhs, rhs) -> Quaternion<S> {
        Quaternion::from_sv(lhs.s * rhs, lhs.v * rhs)
    }
});
 */
