use std::cmp::Ordering;

use float_cmp::Ulps;

use super::{SAdd, Scalar};

#[derive(Copy, Clone)]
pub struct Interval(pub f64, pub f64);
impl Interval {
    pub const EMPTY: Self = Self(f64::INFINITY, f64::NEG_INFINITY);

    fn is_subset_of(self, rhs: Self) -> bool {
        rhs.0 <= self.0 && self.1 <= rhs.1
    }

    fn is_proper_subset_of(self, rhs: Self) -> bool {
        self.is_subset_of(rhs) && self != rhs
    }

    fn is_strict_subset_of(self, rhs: Self) -> bool {
        rhs.0 < self.0 && self.1 < rhs.1
    }

    fn intersection(self, rhs: Self) -> Self {
        if self.1 < rhs.0 || rhs.1 < self.0 {
            Self::EMPTY
        } else {
            Self(self.0.max(rhs.0), self.1.min(rhs.1))
        }
    }

    fn rad(self) -> f64 {
        (self.1 - self.0) / 2.0
    }

    fn mid(self) -> f64 {
        (self.1 + self.0) / 2.0
    }

    fn contains_exact(self, val: f64) -> bool {
        self.0 <= val && self.1 >= val
    }

    fn mig(self) -> f64 {
        if self.contains_exact(0.0) {
            0.0
        } else {
            self.0.abs().min(self.1.abs())
        }
    }

    fn mag(self) -> f64 {
        self.0.abs().max(self.1.abs())
    }

    fn abs(self) -> Self {
        Self(self.mig(), self.mag())
    }

    fn hausdorff(self, rhs: Self) -> f64 {
        (self.0 - rhs.0).abs().max((self.1 - rhs.1).abs())
    }
}
impl PartialEq for Interval {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1
    }
}
impl PartialOrd for Interval {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self == other {
            Some(Ordering::Equal)
        } else if self.0 < other.0 && self.1 < other.1 {
            Some(Ordering::Less)
        } else if self.0 > other.0 && self.1 > other.1 {
            Some(Ordering::Greater)
        } else {
            None
        }
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
impl SAdd<Self> for Interval {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self((self.0 + rhs.0).prev(), (self.1 + rhs.1).next())
    }
}
impl std::fmt::Display for Interval {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("[{}…{}]", self.0, self.1))
    }
}
