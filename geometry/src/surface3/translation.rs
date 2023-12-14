use crate::{Curve3, Curve3Impl, Surface3Impl};

pub struct Translation {
    profile: Curve3,
    path: Curve3,
}
impl Translation {
    pub fn new(profile: Curve3, path: Curve3) -> Self {
        Self { profile, path }
    }
}
impl Surface3Impl for Translation {
    fn u_min(&self) -> f64 {
        self.profile.u_min()
    }

    fn u_max(&self) -> f64 {
        self.profile.u_max()
    }

    fn v_min(&self) -> f64 {
        self.path.u_min()
    }

    fn v_max(&self) -> f64 {
        self.path.u_max()
    }

    fn period_u(&self) -> Option<f64> {
        self.profile.period()
    }

    fn period_v(&self) -> Option<f64> {
        self.path.period()
    }

    fn eval(&self, u: f64, v: f64) -> space::Point3 {
        (self.path.eval(v) + self.profile.eval(u) - self.path.eval(self.path.u_min())).into_point()
    }

    fn der1(&self, u: f64, v: f64) -> (space::Vec3, space::Vec3) {
        let du = self.profile.der1(u);
        let dv = self.path.der1(v);
        (du, dv)
    }

    fn der2(&self, u: f64, v: f64) -> (space::Vec3, space::Vec3, space::Vec3) {
        let duu = self.profile.der2(u);
        let dvv = self.path.der2(v);
        (duu, duu + dvv, dvv)
    }
}
