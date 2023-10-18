use crate::{vec3, Vec3};

#[derive(Debug, Clone)]
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
}
