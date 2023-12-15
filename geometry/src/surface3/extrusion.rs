use space::Vec3;

use crate::{Curve3, Curve3Impl, Surface3Impl};

pub struct Extrusion {
    profile: Curve3,
    direction: Vec3,
}
impl Extrusion {
    pub fn new(profile: Curve3, direction: Vec3) -> Self {
        Self { profile, direction }
    }
}
impl Surface3Impl for Extrusion {
    fn u_min(&self) -> f64 {
        self.profile.u_min()
    }

    fn u_max(&self) -> f64 {
        self.profile.u_max()
    }

    fn v_min(&self) -> f64 {
        0.0
    }

    fn v_max(&self) -> f64 {
        1.0
    }

    fn period_u(&self) -> Option<f64> {
        self.profile.period()
    }

    fn period_v(&self) -> Option<f64> {
        None
    }

    fn eval(&self, u: f64, v: f64) -> space::Point3 {
        Vec3::ZERO.lerp(self.direction, v) + self.profile.eval(u)
    }

    fn der1(&self, u: f64, _v: f64) -> (space::Vec3, space::Vec3) {
        let du = self.profile.der1(u);
        let dv = self.direction;
        (du, dv)
    }

    fn der2(&self, u: f64, _v: f64) -> (space::Vec3, space::Vec3, space::Vec3) {
        let duu = self.profile.der2(u);
        // The dvv vector is [0.0, 0.0, 0.0], so the below is equivalent to
        // (duu, duu + dvv, dvv)
        (duu, duu, Vec3::ZERO)
    }
}
