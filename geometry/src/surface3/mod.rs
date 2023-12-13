mod sweep;

pub use sweep::*;

use space::{Point3, Vec3};

use crate::Curve3;

pub enum Surface3 {
    Sweep(Sweep),
}
impl Surface3 {
    pub fn sweep(profile: Curve3, path: Curve3) -> Self {
        Self::Sweep(Sweep::new(profile, path))
    }
}
impl Surface3Impl for Surface3 {
    fn u_min(&self) -> f64 {
        match self {
            Surface3::Sweep(sweep) => sweep.u_min(),
        }
    }

    fn u_max(&self) -> f64 {
        match self {
            Surface3::Sweep(sweep) => sweep.u_max(),
        }
    }

    fn v_min(&self) -> f64 {
        match self {
            Surface3::Sweep(sweep) => sweep.v_min(),
        }
    }

    fn v_max(&self) -> f64 {
        match self {
            Surface3::Sweep(sweep) => sweep.v_max(),
        }
    }

    fn period_u(&self) -> Option<f64> {
        match self {
            Surface3::Sweep(sweep) => sweep.period_u(),
        }
    }

    fn period_v(&self) -> Option<f64> {
        match self {
            Surface3::Sweep(sweep) => sweep.period_v(),
        }
    }

    fn eval(&self, u: f64, v: f64) -> Point3 {
        match self {
            Surface3::Sweep(sweep) => sweep.eval(u, v),
        }
    }

    fn der1(&self, u: f64, v: f64) -> (Vec3, Vec3) {
        match self {
            Surface3::Sweep(sweep) => sweep.der1(u, v),
        }
    }

    fn der2(&self, u: f64, v: f64) -> (Vec3, Vec3, Vec3) {
        match self {
            Surface3::Sweep(sweep) => sweep.der2(u, v),
        }
    }

    fn u_len(&self) -> f64 {
        match self {
            Surface3::Sweep(sweep) => sweep.u_len(),
        }
    }

    fn v_len(&self) -> f64 {
        match self {
            Surface3::Sweep(sweep) => sweep.v_len(),
        }
    }

    fn is_periodic_u(&self) -> bool {
        match self {
            Surface3::Sweep(sweep) => sweep.is_periodic_u(),
        }
    }

    fn is_periodic_v(&self) -> bool {
        match self {
            Surface3::Sweep(sweep) => sweep.is_periodic_v(),
        }
    }

    fn tangent(&self, u: f64, v: f64) -> (Vec3, Vec3) {
        match self {
            Surface3::Sweep(sweep) => sweep.tangent(u, v),
        }
    }

    fn normal(&self, u: f64, v: f64) -> Vec3 {
        match self {
            Surface3::Sweep(sweep) => sweep.normal(u, v),
        }
    }
}

pub trait Surface3Impl {
    fn u_min(&self) -> f64;
    fn u_max(&self) -> f64;

    fn v_min(&self) -> f64;
    fn v_max(&self) -> f64;

    fn u_len(&self) -> f64 {
        self.u_max() - self.u_min()
    }

    fn v_len(&self) -> f64 {
        self.v_max() - self.v_min()
    }

    fn period_u(&self) -> Option<f64>;

    fn is_periodic_u(&self) -> bool {
        self.period_u().is_some()
    }

    fn period_v(&self) -> Option<f64>;

    fn is_periodic_v(&self) -> bool {
        self.period_v().is_some()
    }

    fn eval(&self, u: f64, v: f64) -> Point3;
    fn der1(&self, u: f64, v: f64) -> (Vec3, Vec3);
    fn der2(&self, u: f64, v: f64) -> (Vec3, Vec3, Vec3);

    fn tangent(&self, u: f64, v: f64) -> (Vec3, Vec3) {
        let (der1_u, der1_v) = self.der1(u, v);
        (der1_u.normalize(), der1_v.normalize())
    }

    fn normal(&self, u: f64, v: f64) -> Vec3 {
        let tangent = self.tangent(u, v);
        tangent.0.cross(tangent.1)
    }
}
