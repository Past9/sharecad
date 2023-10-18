use crate::{vec3, Angle, Vec3};

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
}
