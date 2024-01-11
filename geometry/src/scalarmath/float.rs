use float_cmp::Ulps;

use super::{SArithmetic, Scalar};

#[derive(Copy, Clone)]
pub struct Float(pub f64);
impl Float {
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

    pub fn abs(self) -> Self {
        Self(self.0.abs())
    }
}
impl Scalar for Float {
    fn powi(self, n: i32) -> Self {
        todo!()
    }

    fn sqrt(self) -> Self {
        todo!()
    }
}
impl SArithmetic for Float {
    fn add(self, rhs: Self) -> Self {
        Self(self.0 + rhs.0)
    }

    fn sub(self, rhs: Self) -> Self {
        Self(self.0 - rhs.0)
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
