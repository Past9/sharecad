use std::f64::consts::{PI, TAU};

use space::{point3, vec3, Mat33, Point3, Quat, Vec3};

use crate::Curve3Impl;

#[derive(Debug)]
pub struct Helix {
    /// Radius of the helix
    r: f64,
    /// Axial length (not length along the helical curve)
    /// of one complete revolution of the helix multiplied by 2PI
    h: f64,
    /// Number of revolutions of the helix
    n: f64,

    orientation: Quat,
    translation: Vec3,
}
impl Helix {
    pub fn new(r: f64, h: f64, n: f64, orientation: Quat, translation: Vec3) -> Self {
        Self {
            r,
            h,
            n,
            orientation,
            translation,
        }
    }

    /// Number of revolutions
    pub fn n(&self) -> f64 {
        self.n
    }

    /// Radius
    pub fn r(&self) -> f64 {
        self.r
    }

    pub fn arc_len(&self, u: f64) -> f64 {
        (self.h.powi(2) + self.r.powi(2)).sqrt() * u
    }
}
impl Curve3Impl for Helix {
    fn u_min(&self) -> f64 {
        0.0
    }

    fn u_max(&self) -> f64 {
        self.n * TAU
    }

    fn eval(&self, u: f64) -> Point3 {
        let point = point3(
            self.r * u.cos(), //
            self.r * u.sin(), //
            self.h * u,       //
        );

        self.orientation * point + self.translation
    }

    fn der1(&self, u: f64) -> Vec3 {
        let der1 = vec3(
            self.r * -u.sin(), //
            self.r * u.cos(),  //
            self.h,            //
        );

        self.orientation * der1
    }

    fn der2(&self, u: f64) -> Vec3 {
        let der2 = vec3(
            self.r * -u.cos(), //
            self.r * -u.sin(), //
            0.0,               //
        );

        self.orientation * der2
    }

    fn period(&self) -> Option<f64> {
        Some(TAU)
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts::TAU;

    use space::{assert_cc, deg, point3, vec3, Quat, Vec3};

    use crate::{
        curve3::tests::{validate_der1, validate_der2},
        Curve3Impl, Helix,
    };

    #[test]
    fn arc_length() {
        let helix = Helix::new(1.0, 1.0 / TAU, 1.0, Quat::ZERO, Vec3::ZERO);

        println!("{}", helix.arc_len(TAU));
    }

    fn test_helix() -> Helix {
        Helix::new(
            1.0,
            1.0 / TAU,
            2.0,
            Quat::from_axis_angle(vec3(1.0, 0.0, 0.0), deg(90.0)),
            vec3(1.0, 2.0, 3.0),
        )
    }

    #[test]
    fn helix_points() {
        let points = test_helix().eval_sections(8);

        assert_cc!(point3(2.0, 2.0, 3.0), points[0]);
        assert_cc!(point3(1.0, 1.75, 4.0), points[1]);
        assert_cc!(point3(0.0, 1.5, 3.0), points[2]);
        assert_cc!(point3(1.0, 1.25, 2.0), points[3]);
        assert_cc!(point3(2.0, 1.0, 3.0), points[4]);
        assert_cc!(point3(1.0, 0.75, 4.0), points[5]);
        assert_cc!(point3(0.0, 0.5, 3.0), points[6]);
        assert_cc!(point3(1.0, 0.25, 2.0), points[7]);
        assert_cc!(point3(2.0, 0.0, 3.0), points[8]);
    }

    #[test]
    fn helix_validate_der1() {
        validate_der1(&test_helix(), 100, 1e-7);
    }

    #[test]
    fn helix_validate_der2() {
        validate_der2(&test_helix(), 100, 1e-7);
    }
}
