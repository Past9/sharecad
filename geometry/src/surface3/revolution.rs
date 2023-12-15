use std::f64::consts::TAU;

use space::{rad, vec3, Angle, Mat33, Point3, Vec3};

use crate::{Curve3, Curve3Impl, Surface3Impl};

pub struct Revolution {
    profile: Curve3,
    axis_origin: Point3,
    axis_direction: Vec3,
    start_angle: Angle,
    sweep_angle: Angle,
}
impl Revolution {
    pub fn new(
        profile: Curve3,
        axis_origin: Point3,
        axis_direction: Vec3,
        start_angle: Angle,
        sweep_angle: Angle,
    ) -> Self {
        Self {
            profile,
            axis_origin,
            axis_direction,
            start_angle,
            sweep_angle,
        }
    }

    fn true_v(&self, v: f64) -> f64 {
        (self.start_angle + rad(v)).0
    }
}
impl Surface3Impl for Revolution {
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
        self.sweep_angle.radians()
    }

    fn period_u(&self) -> Option<f64> {
        self.profile.period()
    }

    fn period_v(&self) -> Option<f64> {
        Some(TAU)
    }

    fn eval(&self, u: f64, v: f64) -> space::Point3 {
        let v = self.true_v(v);

        let profile_pos = self.profile.eval(u);

        // Construct a matrix describing the local coordinate system
        // of the rotation
        let x = (profile_pos - self.axis_origin).normalize();
        let z = self.axis_direction.normalize();
        let y = z.cross(x);
        let a = Mat33::from_axes(x, y, z);

        // Construct a matrix that transforms the vector (profile_pos - self.axis_origin)
        // to the rotation's local coordinate system, rotates it around the axis, and
        // then transforms back to global coordinates
        let m =
            a * Mat33::from_axes(
                vec3(v.cos(), v.sin(), 0.0),
                vec3(-v.sin(), v.cos(), 0.0),
                vec3(0.0, 0.0, 1.0),
            ) * a.inverse().unwrap();

        self.axis_origin + m * (profile_pos - self.axis_origin)
    }

    fn der1(&self, u: f64, v: f64) -> (space::Vec3, space::Vec3) {
        let v = self.true_v(v);
        todo!()
    }

    fn der2(&self, u: f64, v: f64) -> (space::Vec3, space::Vec3, space::Vec3) {
        let v = self.true_v(v);
        todo!()
    }
}
