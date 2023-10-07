use auto_ops::{impl_op_ex, impl_op_ex_commutative};

const DEG_TO_RAD: f64 = std::f64::consts::PI / 180.0;
const RAD_TO_DEG: f64 = 180.0 / std::f64::consts::PI;

pub fn deg(deg: f64) -> Angle {
    Angle::deg(deg)
}

pub fn rad(rad: f64) -> Angle {
    Angle::rad(rad)
}

#[derive(Clone, Copy)]
pub struct Angle(pub f64);
impl Angle {
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

    pub fn cos(&self) -> f64 {
        self.0.cos()
    }

    pub fn sin(&self) -> f64 {
        self.0.sin()
    }

    pub fn tan(&self) -> f64 {
        self.0.tan()
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

impl_op_ex!(+|a: Angle, b: Angle| -> Angle { Angle::rad(a.0 + b.0) });
impl_op_ex!(-|a: Angle, b: Angle| -> Angle { Angle::rad(a.0 - b.0) });
impl_op_ex!(/|a: Angle, b: f64| -> Angle { Angle::rad(a.0 / b) });
impl_op_ex_commutative!(*|a: Angle, b: f64| -> Angle { Angle::rad(a.0 * b) });


#[cfg(test)]
mod tests {
    use std::f64::consts::PI;

    use crate::{Angle, rad, deg};

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