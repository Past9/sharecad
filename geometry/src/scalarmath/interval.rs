use std::cmp::Ordering;

use float_cmp::Ulps;

use super::{Float, SArithmetic, Scalar};

pub enum IntervalRelToFloat {
    LessThanFloat,
    ContainsFloat,
    GreaterThanFloat,
}

#[derive(Copy, Clone)]
pub struct Interval(pub Float, pub Float);
impl Interval {
    pub const EMPTY: Self = Self(Float::INFINITY, Float::NEG_INFINITY);

    fn is_empty(self) -> bool {
        self.eq(Self::EMPTY)
    }

    fn is_subset_of(self, rhs: Self) -> bool {
        rhs.0.lte(self.0) && self.1.lte(rhs.1)
    }

    fn is_proper_subset_of(self, rhs: Self) -> bool {
        self.is_subset_of(rhs) && self.neq(rhs)
    }

    fn is_strict_subset_of(self, rhs: Self) -> bool {
        rhs.0.lt(self.0) && self.1.lt(rhs.1)
    }

    fn intersection(self, rhs: Self) -> Self {
        if self.1.lt(rhs.0) || rhs.1.lt(self.0) {
            Self::EMPTY
        } else {
            Self(self.0.max(rhs.0), self.1.min(rhs.1))
        }
    }

    fn rad(self) -> Float {
        self.1.sub(self.0).div(Float(2.0))
    }

    fn mid(self) -> Float {
        self.1.add(self.0).div(Float(2.0))
    }

    fn rel_to_float(self, val: Float) -> IntervalRelToFloat {
        if self.0.lt(val) && self.1.lt(val) {
            IntervalRelToFloat::LessThanFloat
        } else if self.0.gt(val) && self.1.gt(val) {
            IntervalRelToFloat::GreaterThanFloat
        } else {
            IntervalRelToFloat::ContainsFloat
        }
    }

    fn contains_exact(self, val: Float) -> bool {
        self.0.lte(val) && self.1.gte(val)
    }

    fn contains_zero(self) -> bool {
        self.contains_exact(Float(0.0))
    }

    fn mig(self) -> Float {
        if self.contains_zero() {
            Float(0.0)
        } else {
            self.0.abs().min(self.1.abs())
        }
    }

    fn mag(self) -> Float {
        self.0.abs().max(self.1.abs())
    }

    fn abs(self) -> Self {
        Self(self.mig(), self.mag())
    }

    fn hausdorff(self, rhs: Self) -> Float {
        self.0.sub(rhs.0).abs().max(self.1.sub(rhs.1).abs())
    }
}
impl Scalar for Interval {
    fn powi(self, n: i32) -> Self {
        todo!()
    }

    fn sqrt(self) -> Self {
        todo!()
    }
}
impl SArithmetic for Interval {
    fn add(self, rhs: Self) -> Self {
        if self.is_empty() || rhs.is_empty() {
            return Self::EMPTY;
        }

        Self(self.0.add(rhs.0).prev(), self.1.add(rhs.1).next())
    }

    fn sub(self, rhs: Self) -> Self {
        if self.is_empty() || rhs.is_empty() {
            return Self::EMPTY;
        }

        Self(self.0.sub(rhs.1).prev(), self.1.sub(rhs.0).next())
    }

    fn mul(self, rhs: Self) -> Self {
        if self.is_empty() || rhs.is_empty() {
            return Self::EMPTY;
        }

        Self(
            self.0
                .mul(rhs.0)
                .prev()
                .min(self.0.mul(rhs.1).prev())
                .min(self.1.mul(rhs.0).prev())
                .min(self.1.mul(rhs.1).prev()),
            self.0
                .mul(rhs.0)
                .next()
                .max(self.0.mul(rhs.1).next())
                .max(self.1.mul(rhs.0).next())
                .max(self.1.mul(rhs.1).next()),
        )
    }

    fn div(self, rhs: Self) -> Self {
        /*
        Self(
            self.0
                .div(rhs.0)
                .prev()
                .min(self.0.div(rhs.1).prev())
                .min(self.1.div(rhs.0).prev())
                .min(self.1.div(rhs.1).prev()),
            self.0
                .div(rhs.0)
                .next()
                .max(self.0.div(rhs.1).next())
                .max(self.1.div(rhs.0).next())
                .max(self.1.div(rhs.1).next()),
        )
         */

        let a = self;
        let b = rhs;

        if !b.contains_zero() {
            a.mul(Self(Float(1.0).div(b.1), Float(1.0).div(b.0)))
        } else if a.contains_zero() && b.contains_zero() {
            Self(Float::NEG_INFINITY, Float::INFINITY)
        } else if a.1.lt(Float(0.0)) {
            if b.0.lt(b.1) && b.1.eq(Float(0.0)) {
                Self(a.1.div(b.0), Float::INFINITY)
            } else if b.0.lt(Float(0.0)) && b.1.gt(Float(0.0)) {
                Self(a.1.div(b.0), a.1.div(b.1))
            } else if Float(0.0).eq(b.0) && b.0.lt(b.1) {
                Self(Float::NEG_INFINITY, a.1.div(b.1))
            } else {
                panic!("Div case 1");
            }
        } else if Float(0.0).lt(a.0) {
            if b.0.lt(b.1) && b.1.eq(Float(0.0)) {
                Self(Float::NEG_INFINITY, a.0.div(b.0))
            } else if b.0.lt(Float(0.0)) && b.1.gt(Float(0.0)) {
                Self(a.0.div(b.1), a.0.div(b.0))
            } else if Float(0.0).eq(b.0) && b.0.lt(b.1) {
                Self(a.0.div(b.1), Float::INFINITY)
            } else {
                panic!("Div case 2")
            }
        } else if !a.contains_zero() {
            Self::EMPTY
        } else {
            panic!("Div case 3")
        }
    }

    fn eq(self, rhs: Self) -> bool {
        self.0.eq(rhs.0) && self.1.eq(rhs.1)
    }

    fn neq(self, rhs: Self) -> bool {
        !self.eq(rhs)
    }

    fn lt(self, rhs: Self) -> bool {
        self.1.lt(rhs.0)
    }

    fn lte(self, rhs: Self) -> bool {
        self.0.lte(rhs.0) && self.1.eq(rhs.1)
    }

    fn gt(self, rhs: Self) -> bool {
        self.0.gt(rhs.1)
    }

    fn gte(self, rhs: Self) -> bool {
        self.0.eq(rhs.0) && self.1.gte(rhs.1)
    }
}
impl std::fmt::Display for Interval {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("[{}…{}]", self.0, self.1))
    }
}
