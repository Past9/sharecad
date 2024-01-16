use auto_ops::{impl_op_ex, impl_op_ex_commutative};
use gen_ops::gen_ops;
use std::f64::consts::PI;

use super::Scalar;

pub fn deg<S: Scalar>(deg: S) -> Angle<S> {
    Angle::deg(deg)
}

pub fn rad<S: Scalar>(rad: S) -> Angle<S> {
    Angle::rad(rad)
}

#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub struct Angle<S: Scalar>(pub S);
impl<S: Scalar> Angle<S> {
    pub const ZERO: Self = Self(S::ZERO);
    pub const RAD_PI: Self = Self(S::PI);
    pub const RAD_2PI: Self = Self(S::TAU);
    pub const RAD_1_2_PI: Self = Self(S::FRAC_PI_2);
    pub const RAD_1_4_PI: Self = Self(S::FRAC_PI_4);

    pub const DEG_180: Self = Self(S::PI);
    pub const DEG_360: Self = Self(S::TAU);
    pub const DEG_90: Self = Self(S::FRAC_PI_2);
    pub const DEG_45: Self = Self(S::FRAC_PI_8);

    pub fn is_zero(&self) -> bool {
        self.0 == S::ZERO
    }

    pub fn deg(deg: S) -> Self {
        Self(deg * S::PI / S::exact(180.0))
    }

    pub fn rad(rad: S) -> Self {
        Self(rad)
    }

    pub fn degrees(&self) -> S {
        self.0 * S::exact(180.0) / S::PI
    }

    pub fn radians(&self) -> S {
        self.0
    }

    pub fn sin(&self) -> S {
        self.0.sin()
    }

    pub fn cos(&self) -> S {
        self.0.cos()
    }

    pub fn tan(&self) -> S {
        self.0.tan()
    }

    pub fn csc(&self) -> S {
        self.0.sin().recip()
    }

    pub fn sec(&self) -> S {
        self.0.cos().recip()
    }

    pub fn cot(&self) -> S {
        self.0.tan().recip()
    }

    pub fn sin_cos(&self) -> (S, S) {
        self.0.sin_cos()
    }

    /*
    // Angle from `self` to `other` going counterclockwise
    pub fn angle_ccw(&self, other: Self) -> Self {
        (other.normalize() - self.normalize()).normalize()
    }

    // Makes the angle always between 0 and 2pi
    pub fn normalize(&self) -> Self {
        let pi_2 = S::TAU;
        let mut rads = self.0 % pi_2;
        if rads < S::ZERO {
            rads += pi_2;
        }

        rad(rads)
    }
     */
}
impl<S: Scalar> From<Angle<S>> for f64 {
    fn from(value: Angle<S>) -> Self {
        value.0.as_f64()
    }
}
impl<S: Scalar> std::fmt::Debug for Angle<S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}ᶜ", self.0))
    }
}
impl<S: Scalar> std::fmt::Display for Angle<S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}ᶜ", self.0))
    }
}

gen_ops!(
    <S>;
    types Angle<S> => Angle<S>;
    for - call |a: &Angle<S>| {
        Angle(-a.0)
    };
    where S: Scalar
);

gen_ops!(
    <S>;
    types Angle<S>, Angle<S> => Angle<S>;
    for + call |l: &Angle<S>, r: &Angle<S>| {
        Angle(l.0 + r.0)
    };
    for - call |l: &Angle<S>, r: &Angle<S>| {
        Angle(l.0 - r.0)
    };
    where S: Scalar
);

gen_ops!(
    <S>;
    types Angle<S>, S => Angle<S>;
    for * call |l: &Angle<S>, r: &S| {
        Angle(l.0 * *r)
    };
    for / call |l: &Angle<S>, r: &S| {
        Angle(l.0 / *r)
    };
    where S: Scalar
);

#[cfg(test)]
mod tests {
    use std::f64::consts::PI;

    use crate::math::{deg, rad, Angle};

    /*
    #[test]
    fn angle_ccw() {
        assert_cc!(deg(90.0), deg(315.0).angle_ccw(deg(45.0)));
        assert_cc!(deg(270.0), deg(45.0).angle_ccw(deg(315.0)));
        assert_cc!(deg(0.0), deg(45.0).angle_ccw(deg(45.0)));
        assert_cc!(deg(0.0), deg(45.0).angle_ccw(deg(405.0)));
        assert_cc!(deg(0.0), deg(405.0).angle_ccw(deg(765.0)));
    }
     */

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
