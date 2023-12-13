use crate::{Curve3, Curve3Impl, Surface3Impl};

pub struct Sweep {
    profile: Curve3,
    path: Curve3,
}
impl Sweep {
    pub fn new(profile: Curve3, path: Curve3) -> Self {
        Self { profile, path }
    }
}
impl Surface3Impl for Sweep {
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
        let translation = self.path.eval(v);
        self.profile.eval(u) + translation
    }

    fn der1(&self, u: f64, v: f64) -> (space::Vec3, space::Vec3) {
        (self.profile.der1(u), self.path.der1(v))
    }

    fn der2(&self, u: f64, v: f64) -> (space::Vec3, space::Vec3, space::Vec3) {
        let profile_der2 = self.profile.der2(u);
        let path_der2 = self.path.der2(v);
        (profile_der2, profile_der2 + path_der2, path_der2)
    }
}
