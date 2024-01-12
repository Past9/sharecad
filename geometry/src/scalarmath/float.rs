use std::ops::Rem;

use auto_ops::impl_op_ex;
use float_cmp::Ulps;

use super::{SArithmetic, Scalar};

#[derive(Copy, Clone, PartialEq)]
pub struct Float(pub f64);
impl Float {
    pub const NAN: Self = Self(f64::NAN);
    pub const INFINITY: Self = Self(f64::INFINITY);
    pub const NEG_INFINITY: Self = Self(f64::NEG_INFINITY);

    pub fn prev(self) -> Self {
        Self(self.0.prev())
    }

    pub fn next(self) -> Self {
        Self(self.0.next())
    }

    pub fn min(self, rhs: Self) -> Self {
        Self(self.0.min(rhs.0))
    }

    pub fn max(self, rhs: Self) -> Self {
        Self(self.0.max(rhs.0))
    }

    pub fn rem(self, rhs: Self) -> Self {
        Self(self.0.rem(rhs.0))
    }
}
impl Scalar for Float {
    const E: Self = Self(std::f64::consts::E);
    const FRAC_1_PI: Self = Self(std::f64::consts::FRAC_1_PI);
    const FRAC_1_SQRT_2: Self = Self(std::f64::consts::FRAC_1_SQRT_2);
    const FRAC_2_PI: Self = Self(std::f64::consts::FRAC_2_PI);
    const FRAC_2_SQRT_PI: Self = Self(std::f64::consts::FRAC_2_SQRT_PI);
    const FRAC_PI_2: Self = Self(std::f64::consts::FRAC_PI_2);
    const FRAC_PI_3: Self = Self(std::f64::consts::FRAC_PI_3);
    const FRAC_PI_4: Self = Self(std::f64::consts::FRAC_PI_4);
    const FRAC_PI_6: Self = Self(std::f64::consts::FRAC_PI_6);
    const FRAC_PI_8: Self = Self(std::f64::consts::FRAC_PI_8);
    const LN_10: Self = Self(std::f64::consts::LN_10);
    const LN_2: Self = Self(std::f64::consts::LN_2);
    const LOG10_2: Self = Self(std::f64::consts::LOG10_2);
    const LOG10_E: Self = Self(std::f64::consts::LOG10_E);
    const LOG2_10: Self = Self(std::f64::consts::LOG2_10);
    const LOG2_E: Self = Self(std::f64::consts::LOG2_E);
    const PI: Self = Self(std::f64::consts::PI);
    const SQRT_2: Self = Self(std::f64::consts::SQRT_2);
    const TAU: Self = Self(std::f64::consts::TAU);

    fn powi(self, n: i32) -> Self {
        Self(self.0.powi(n))
    }

    fn sqrt(self) -> Self {
        Self(self.0.sqrt())
    }

    fn exp(self) -> Self {
        Self(self.0.exp())
    }

    fn abs(self) -> Self {
        Self(self.0.abs())
    }

    fn atan(self) -> Self {
        Self(self.0.atan())
    }

    fn sin(self) -> Self {
        Self(self.0.sin())
    }

    fn cos(self) -> Self {
        Self(self.0.cos())
    }

    fn tan(self) -> Self {
        Self(self.0.tan())
    }
}
impl SArithmetic for Float {
    fn neg(self) -> Self {
        Self(-self.0)
    }

    fn mul(self, rhs: Self) -> Self {
        Self(self.0 * rhs.0)
    }

    fn div(self, rhs: Self) -> Self {
        Self(self.0 / rhs.0)
    }

    fn eq(self, rhs: Self) -> bool {
        self.0 == rhs.0
    }

    fn neq(self, rhs: Self) -> bool {
        self.0 != rhs.0
    }

    fn lt(self, rhs: Self) -> bool {
        self.0 < rhs.0
    }

    fn lte(self, rhs: Self) -> bool {
        self.0 <= rhs.0
    }

    fn gt(self, rhs: Self) -> bool {
        self.0 > rhs.0
    }

    fn gte(self, rhs: Self) -> bool {
        self.0 >= rhs.0
    }
}
impl std::fmt::Display for Float {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl_op_ex!(+|l: &Float, r: &Float| -> Float { Float(l.0 + r.0) });
impl_op_ex!(-|l: &Float, r: &Float| -> Float { Float(l.0 - r.0) });
