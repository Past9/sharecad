use crate::Surface3Impl;

pub struct Revolution {}
impl Surface3Impl for Revolution {
    fn u_min(&self) -> f64 {
        todo!()
    }

    fn u_max(&self) -> f64 {
        todo!()
    }

    fn v_min(&self) -> f64 {
        todo!()
    }

    fn v_max(&self) -> f64 {
        todo!()
    }

    fn period_u(&self) -> Option<f64> {
        todo!()
    }

    fn period_v(&self) -> Option<f64> {
        todo!()
    }

    fn eval(&self, u: f64, v: f64) -> space::Point3 {
        todo!()
    }

    fn der1(&self, u: f64, v: f64) -> (space::Vec3, space::Vec3) {
        todo!()
    }

    fn der2(&self, u: f64, v: f64) -> (space::Vec3, space::Vec3, space::Vec3) {
        todo!()
    }
}
