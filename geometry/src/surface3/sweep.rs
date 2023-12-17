use std::cell::OnceCell;

use space::{Mat33, Point3, Vec3};

use crate::{Curve3, Curve3Impl, Surface3Impl};

pub struct Sweep {
    profile: Curve3,
    path: Curve3,
}
impl Sweep {
    pub fn new(profile: Curve3, path: Curve3) -> Self {
        Self { profile, path }
    }

    fn path_axes(&self, v: f64) -> (Mat33, Mat33, Mat33) {
        let der1 = self.path.der1(v);
        let d = self.path.never_tangent();

        // Compute axes of local coordinate system
        let (i1, i2, i3, d2) = {
            let i1 = der1.normalize();

            let d2 = d - (i1.dot(d)) * i1;

            let i2 = d2.normalize();
            let i3 = i1.cross(i2);

            (i1, i2, i3, d2)
        };

        let der2 = self.path.der2(v);

        // Compute first derivatives of axes
        let (i1_der1, i2_der1, i3_der1, d2_der1) = {
            let i1_der1 = der1.norm_der1(der2);

            //let d2_der1 = -i1 * (i1_der1.dot(d));
            let d2_der1 = (-i1_der1.dot(d) * i1) - (i1.dot(d) * i1_der1);
            let i2_der1 = d2.norm_der1(d2_der1);

            let i3_der1 = i1.cross(i2_der1) + i1_der1.cross(i2);

            (i1_der1, i2_der1, i3_der1, d2_der1)
        };

        // Compute second derivatives of axes
        let (i1_der2, i2_der2, i3_der2) = {
            let der3 = self.path.der3(v);

            let i1_der2 = der1.norm_der2(der2, der3);

            //let d2_der2 = -i1 * (i1_der2.dot(d));
            let d2_der2 =
                (-i1_der2.dot(d) * i1) - 2.0 * (i1_der1.dot(d) * i1_der1) - (i1.dot(d) * i1_der2);
            let i2_der2 = d2.norm_der2(d2_der1, d2_der2);

            let i3_der2 = i1.cross(i2_der2) + 2.0 * i1_der1.cross(i2_der2) + i2_der2.cross(i2);

            (i1_der2, i2_der2, i3_der2)
        };

        (
            Mat33::from_axes(i1, i2, i3),
            Mat33::from_axes(i1_der1, i2_der1, i3_der1),
            Mat33::from_axes(i1_der2, i2_der2, i3_der2),
        )
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

    fn eval(&self, u: f64, v: f64) -> Point3 {
        let profile_pos = self.profile.eval(u);
        let path_start = self.path.eval(self.v_min());
        let path_pos = self.path.eval(v);

        let m = self.path_axes(v).0 * self.path_axes(self.v_min()).0.inverse().unwrap();

        path_pos + m * (profile_pos - path_start)
    }

    fn der1(&self, u: f64, v: f64) -> (Vec3, Vec3) {
        let path_start = self.path.eval(self.v_min());
        let path_axes_start_inverse = self.path_axes(self.v_min()).0.inverse().unwrap();
        let path_axes = self.path_axes(v);

        let m = path_axes.0 * path_axes_start_inverse;
        let du = m * self.profile.der1(u);

        let m_der1 = path_axes.1 * path_axes_start_inverse;

        let dv = self.path.der1(v) + m_der1 * (self.profile.eval(u) - path_start);

        (du, dv)
    }

    fn der2(&self, u: f64, v: f64) -> (Vec3, Vec3, Vec3) {
        let profile_pos = self.profile.eval(u);
        let path_start = self.path.eval(self.v_min());
        let path_axes_start_inverse = self.path_axes(self.v_min()).0.inverse().unwrap();
        let path_axes = self.path_axes(v);

        //println!("path_axes ({}, {}) = {:#?}", u, v, path_axes);

        let m = path_axes.0 * path_axes_start_inverse;
        let duu = m * self.profile.der2(u);

        let m_der1 = path_axes.1 * path_axes_start_inverse;
        let duv = m_der1 * self.profile.der1(u);

        let m_der2 = path_axes.2 * path_axes_start_inverse;
        let dvv = self.path.der2(v) + m_der2 * (profile_pos - path_start);

        (duu, duv, dvv)
    }
}

#[cfg(test)]
mod tests {
    use space::{deg, vec3, Quat, Vec3};

    use crate::{surface3::tests::validate_ders_2d, Curve3, Surface3};

    fn test_sweep() -> Surface3 {
        Surface3::sweep(
            // Profile
            Curve3::arc(1.0, deg(90.0), Quat::ZERO, vec3(1.0, 0.0, 0.0)),
            // Path
            Curve3::arc(
                1.0,
                deg(135.0),
                Quat::from_axis_angle(Vec3::UNIT_X, deg(90.0)),
                vec3(-2.0, 0.0, 0.0),
            ),
        )
    }

    #[test]
    pub fn sweep_validate_ders() {
        let sweep = test_sweep();

        validate_ders_2d(&sweep, 100, 1e-7)
    }
}
