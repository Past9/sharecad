use space::{Mat33, Vec3};

use crate::{Curve3, Curve3Impl, Surface3Impl};

pub struct Sweep {
    profile: Curve3,
    path: Curve3,
}
impl Sweep {
    pub fn new(profile: Curve3, path: Curve3) -> Self {
        Self { profile, path }
    }

    pub fn path_translation(&self, v: f64) -> Vec3 {
        let start_pos = self.path.eval(self.path.u_min());
        let cur_pos = self.path.eval(v);
        cur_pos - start_pos
    }

    pub fn path_rotation(&self, v: f64) -> Mat33 {
        let start_rot = self.path.frenet(self.path.u_min());
        let cur_rot = self.path.frenet(v);
        //start_rot.inverse().unwrap() * cur_rot
        start_rot.inverse().unwrap() * cur_rot
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
        let frenet = self.path.frenet(v);
        let profile_pos = self.profile.eval(u);
        let path_pos = self.path.eval(v);
        path_pos + frenet * profile_pos.into_vec()
    }

    fn der1(&self, u: f64, v: f64) -> (space::Vec3, space::Vec3) {
        let frenet = self.path.frenet(v);
        let du = frenet * self.profile.der1(u);
        let dv = self.path.der1(v); // * self.profile.eval(u).into_vec();
        (du, dv)
    }

    fn der2(&self, u: f64, v: f64) -> (space::Vec3, space::Vec3, space::Vec3) {
        let frenet = self.path.frenet(v);
        let profile_der2 = self.profile.der2(u);
        let path_der2 = self.path.der2(v);

        let duu = frenet * profile_der2;
        let dvv = path_der2;
        let duv = duu + dvv;

        (duu, duv, dvv)
    }
}

#[cfg(test)]
mod tests {
    use space::{deg, vec3, Quat, Vec3};

    use crate::{surface3::tests::validate_der1, Curve3, Curve3Impl, Surface3};

    #[test]
    pub fn test_der1() {
        let profile = Curve3::arc(
            1.0,
            deg(90.0),
            Quat::from_axis_angle(Vec3::UNIT_X, deg(-90.0)),
            Vec3::ZERO,
        );

        let path = Curve3::arc(
            1.0,
            deg(180.0),
            Quat::from_axis_angle(Vec3::UNIT_X, deg(90.0)),
            vec3(0.0, 0.0, 0.0),
        );

        let sweep = Surface3::sweep(profile.clone(), path.clone());

        validate_der1(&sweep, 100, 1e-7);
    }
}
