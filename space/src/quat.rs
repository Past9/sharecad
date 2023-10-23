use crate::{vec3, Angle, Mat33, Mat44, Point3, Vec3};
use auto_ops::{impl_op_ex, impl_op_ex_commutative};

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

    pub fn dot(&self, other: Self) -> f64 {
        self.s * other.s + self.v.dot(other.v)
    }

    pub fn magnitude2(&self) -> f64 {
        self.dot(*self)
    }

    pub fn magnitude(&self) -> f64 {
        self.magnitude2().sqrt()
    }

    pub fn normalize(&self) -> Self {
        let mag = self.magnitude();
        Self {
            v: self.v / mag,
            s: self.s / mag,
        }
    }
}

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

/*
impl_operator!(<S: BaseFloat> Mul<S> for Quaternion<S> {
    fn mul(lhs, rhs) -> Quaternion<S> {
        Quaternion::from_sv(lhs.s * rhs, lhs.v * rhs)
    }
});
 */
