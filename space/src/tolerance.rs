use std::borrow::Borrow;

use crate::{Point2, Vec2};

pub const COINCIDENT_TOL: f64 = 1e-10;

/// Asserts that two expressions are geometrically coincident. Requires that the left
/// expression's type implements `space::tolerance::Coincident<{right expression type}>`.
#[allow(unused_macros)]
#[macro_export]
macro_rules! assert_cc {
    ($a:expr, $b:expr) => {
        assert!(
            $crate::Coincidence::cc(&$a, $b),
            "assertion failed: `left.is_coincident(right)`\n  left: `{:?}`\n right: `{:?}`",
            $a,
            $b
        )
    };
}

/// Asserts that two expressions are not geometrically coincident. Requires that the left
/// expression's type implements `space::tolerance::Coincident<{right expression type}>`.
#[allow(unused_macros)]
#[macro_export]
macro_rules! assert_nc {
    ($a:expr, $b:expr) => {
        assert!(
            !crate::Coincidence::cc(&$a, $b),
            "assertion failed: `!left.is_coincident(right)`\n  left: `{:?}`\n right: `{:?}`",
            $a,
            $b
        )
    };
}

/// Checks whether the absolute value of the difference between `a` and `b`
/// is less than or equal to `tolerance`
pub fn within_tolerance_f32(a: f32, b: f32, tolerance: f32) -> bool {
    (a - b).abs() <= tolerance
}

/// Checks whether the absolute value of the difference between `a` and `b`
/// is less than or equal to `tolerance`
pub fn within_tolerance_f64(a: f64, b: f64, tolerance: f64) -> bool {
    (a - b).abs() <= tolerance
}

pub trait Coincidence<T> {
    fn cc(&self, other: T) -> bool;
}

impl Coincidence<f32> for f32 {
    /// To be considered geometrically coincident,
    /// `f32`s must have a difference near `0.0`
    fn cc(&self, other: f32) -> bool {
        within_tolerance_f32(*self, other, COINCIDENT_TOL as f32)
    }
}
impl Coincidence<f64> for f64 {
    /// To be considered geometrically coincident,
    /// `f64`s must have a difference near `0.0`
    fn cc(&self, other: f64) -> bool {
        within_tolerance_f64(*self, other, COINCIDENT_TOL)
    }
}

impl<T: Borrow<Vec2>> Coincidence<T> for Vec2 {
    /// To be considered geometrically coincident,
    /// vectors are treated as points and those points
    /// must be separated by a distance near `0.0`
    fn cc(&self, other: T) -> bool {
        self.to_point().cc(other.borrow().to_point())
    }
}

impl<T: Borrow<Point2>> Coincidence<T> for Point2 {
    /// To be considered geometrically coincident,
    /// points must be separated by a distance near `0.0`
    fn cc(&self, other: T) -> bool {
        within_tolerance_f64((*self - *other.borrow()).magnitude(), 0.0, COINCIDENT_TOL)
    }
}

#[cfg(test)]
mod tests {
    use crate::{point2, vec2};

    use super::*;

    #[test]
    fn checks_within_tolerance() {
        assert!(within_tolerance_f64(1.0, 1.01, 0.1));
        assert!(!within_tolerance_f64(1.0, 1.01, 0.001));
        assert!(within_tolerance_f64(-1.0, -1.01, 0.1));
        assert!(!within_tolerance_f64(-1.0, -1.01, 0.001));
    }

    #[test]
    fn f64_coincidence() {
        assert_cc!(1.0, 1.0);
        assert_cc!(1.0, 1.0 + 1e-11);
        assert_nc!(1.0, 1.0 + 1e-9);

        assert_cc!(1.0, 1.0);
        assert_cc!(1.0, 1.0 - 1e-11);
        assert_nc!(1.0, 1.0 - 1e-9);

        assert_cc!(-1.0, -1.0);
        assert_cc!(-1.0, -1.0 + 1e-11);
        assert_nc!(-1.0, -1.0 + 1e-9);

        assert_cc!(-1.0, -1.0);
        assert_cc!(-1.0, -1.0 - 1e-11);
        assert_nc!(-1.0, -1.0 - 1e-9);
    }

    #[test]
    fn vec2_coincidence() {
        assert_cc!(vec2(1.0, 1.0), vec2(1.0, 1.0));
        assert_cc!(vec2(1.0, -1.0), vec2(1.0, -1.0));
        assert_cc!(vec2(-1.0, 1.0), vec2(-1.0, 1.0));
        assert_cc!(vec2(-1.0, -1.0), vec2(-1.0, -1.0));
        assert_cc!(vec2(0.0, 1.0), vec2(0.0, 1.0));
        assert_cc!(vec2(1.0, 0.0), vec2(1.0, 0.0));
        assert_cc!(vec2(0.0, 0.0), vec2(0.0, 0.0));

        assert_cc!(vec2(1.0, 1.0), vec2(1.0 + 1e-11, 1.0));
        assert_nc!(vec2(1.0, 1.0), vec2(1.0 + 1e-9, 1.0));
    }

    #[test]
    fn point2_coincidence() {
        assert_cc!(point2(1.0, 1.0), point2(1.0, 1.0));
        assert_cc!(point2(1.0, -1.0), point2(1.0, -1.0));
        assert_cc!(point2(-1.0, 1.0), point2(-1.0, 1.0));
        assert_cc!(point2(-1.0, -1.0), point2(-1.0, -1.0));
        assert_cc!(point2(0.0, 1.0), point2(0.0, 1.0));
        assert_cc!(point2(1.0, 0.0), point2(1.0, 0.0));
        assert_cc!(point2(0.0, 0.0), point2(0.0, 0.0));

        assert_cc!(point2(1.0, 1.0), point2(1.0 + 1e-11, 1.0));
        assert_nc!(point2(1.0, 1.0), point2(1.0 + 1e-9, 1.0));
    }
}
