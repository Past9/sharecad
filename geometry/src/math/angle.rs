use auto_ops::{impl_op_ex, impl_op_ex_commutative};
use std::f64::consts::PI;

const PI2_1: f64 = PI * 2.0;
const PI1_2: f64 = PI / 2.0;
const PI1_4: f64 = PI / 4.0;
const DEG_TO_RAD: f64 = PI / 180.0;
const RAD_TO_DEG: f64 = 180.0 / PI;

pub fn deg(deg: f64) -> Angle {
    Angle::deg(deg)
}

pub fn rad(rad: f64) -> Angle {
    Angle::rad(rad)
}

#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub struct Angle(pub f64);
impl Angle {
    pub const ZERO: Self = Self(0.0);
    pub const RAD_PI: Self = Self(PI);
    pub const RAD_2PI: Self = Self(PI2_1);
    pub const RAD_1_2_PI: Self = Self(PI1_2);
    pub const RAD_1_4_PI: Self = Self(PI1_4);

    pub const DEG_180: Self = Self(PI);
    pub const DEG_360: Self = Self(PI2_1);
    pub const DEG_90: Self = Self(PI1_2);
    pub const DEG_45: Self = Self(PI1_4);

    pub fn is_zero(&self) -> bool {
        self.0 == 0.0
    }

    pub fn deg(deg: f64) -> Self {
        Self(deg * DEG_TO_RAD)
    }

    pub fn rad(rad: f64) -> Self {
        Self(rad)
    }

    pub fn degrees(&self) -> f64 {
        self.0 * RAD_TO_DEG
    }

    pub fn radians(&self) -> f64 {
        self.0
    }

    pub fn sin(&self) -> f64 {
        self.0.sin()
    }

    pub fn cos(&self) -> f64 {
        self.0.cos()
    }

    pub fn tan(&self) -> f64 {
        self.0.tan()
    }

    pub fn csc(&self) -> f64 {
        self.0.sin().recip()
    }

    pub fn sec(&self) -> f64 {
        self.0.cos().recip()
    }

    pub fn cot(&self) -> f64 {
        self.0.tan().recip()
    }

    pub fn sin_cos(&self) -> (f64, f64) {
        self.0.sin_cos()
    }

    // Angle from `self` to `other` going counterclockwise
    pub fn angle_ccw(&self, other: Self) -> Angle {
        (other.normalize() - self.normalize()).normalize()
    }

    // Makes the angle always between 0 and 2pi
    pub fn normalize(&self) -> Self {
        let pi_2 = PI2_1;
        let mut rads = self.0 % pi_2;
        if rads < 0.0 {
            rads += pi_2;
        }

        rad(rads)
    }
}
impl From<Angle> for f64 {
    fn from(value: Angle) -> Self {
        value.0
    }
}
impl std::fmt::Debug for Angle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}ᶜ", self.0))
    }
}
impl std::fmt::Display for Angle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}ᶜ", self.0))
    }
}

impl_op_ex!(-|a: Angle| -> Angle { Angle::rad(-a.0) });
impl_op_ex!(+|a: Angle, b: Angle| -> Angle { Angle::rad(a.0 + b.0) });
impl_op_ex!(-|a: Angle, b: Angle| -> Angle { Angle::rad(a.0 - b.0) });
impl_op_ex!(/|a: Angle, b: f64| -> Angle { Angle::rad(a.0 / b) });
impl_op_ex!(/|a: Angle, b: Angle| -> f64 { a.0 / b.0 });
impl_op_ex_commutative!(*|a: Angle, b: f64| -> Angle { Angle::rad(a.0 * b) });

#[cfg(test)]
mod tests {
    use std::f64::consts::PI;

    use crate::math::{deg, rad, Angle};

    #[test]
    fn angle_ccw() {
        assert_cc!(deg(90.0), deg(315.0).angle_ccw(deg(45.0)));
        assert_cc!(deg(270.0), deg(45.0).angle_ccw(deg(315.0)));
        assert_cc!(deg(0.0), deg(45.0).angle_ccw(deg(45.0)));
        assert_cc!(deg(0.0), deg(45.0).angle_ccw(deg(405.0)));
        assert_cc!(deg(0.0), deg(405.0).angle_ccw(deg(765.0)));
    }

    #[test]
    fn is_rad_internally() {
        let angle = Angle(1.0);
        assert_eq!(angle.0, angle.radians());
    }

    #[test]
    fn converts_rad_to_deg() {
        let angle = rad(PI);
        assert_cc!(180.0, angle.degrees());
    }

    #[test]
    fn converts_deg_to_rad() {
        let angle = deg(180.0);
        assert_cc!(PI, angle.radians());
    }
}
