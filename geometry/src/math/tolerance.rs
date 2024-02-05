use super::{Angle, Interval, Mat22, Mat33, Mat44, Scalar, Vec2, Vec3, Vec4};

pub const COINCIDENT_TOL: f64 = 1e-10;
pub const NEWTON_TOL: f64 = COINCIDENT_TOL * 0.1;

/// Asserts that two expressions are geometrically coincident. Requires that the left
/// expression's type implements `space::tolerance::Coincident<{right expression type}>`.
#[allow(unused_macros)]
#[macro_export]
macro_rules! assert_cc {
    ($a:expr, $b:expr) => {
        assert!(
            $crate::math::Coincidence::cc($a, $b),
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
            !crate::math::Coincidence::cc($a, $b),
            "assertion failed: `!left.is_coincident(right)`\n  left: `{:?}`\n right: `{:?}`",
            $a,
            $b
        )
    };
}

pub trait Coincidence<S: Scalar>
where
    Self: Sized,
{
    fn cc_tol(self, other: Self, tolerance: f64) -> bool;
    fn cc(self, other: Self) -> bool {
        self.cc_tol(other, COINCIDENT_TOL)
    }
}

impl Coincidence<f64> for f64 {
    fn cc_tol(self, other: Self, tolerance: f64) -> bool {
        (self - other).abs() <= tolerance
    }
}

impl Coincidence<Interval> for Interval {
    fn cc_tol(self, other: Self, tolerance: f64) -> bool {
        self.mid().cc_tol(other.mid(), tolerance)
    }
}

impl<S: Scalar> Coincidence<S> for Vec2<S> {
    fn cc_tol(self, other: Self, tolerance: f64) -> bool {
        self.x.cc_tol(other.x, tolerance) && self.y.cc_tol(other.y, tolerance)
    }
}

impl<S: Scalar> Coincidence<S> for Vec3<S> {
    fn cc_tol(self, other: Self, tolerance: f64) -> bool {
        self.x.cc_tol(other.x, tolerance)
            && self.y.cc_tol(other.y, tolerance)
            && self.z.cc_tol(other.z, tolerance)
    }
}

impl<S: Scalar> Coincidence<S> for Vec4<S> {
    fn cc_tol(self, other: Self, tolerance: f64) -> bool {
        self.x.cc_tol(other.x, tolerance)
            && self.y.cc_tol(other.y, tolerance)
            && self.z.cc_tol(other.z, tolerance)
            && self.w.cc_tol(other.w, tolerance)
    }
}

impl<S: Scalar> Coincidence<S> for Mat22<S> {
    fn cc_tol(self, other: Self, tolerance: f64) -> bool {
        self[0][0].cc_tol(other[0][0], tolerance)
            && self[0][1].cc_tol(other[0][1], tolerance)
            && self[1][0].cc_tol(other[1][0], tolerance)
            && self[1][1].cc_tol(other[1][1], tolerance)
    }
}

impl<S: Scalar> Coincidence<S> for Mat33<S> {
    fn cc_tol(self, other: Self, tolerance: f64) -> bool {
        self[0][0].cc_tol(other[0][0], tolerance)
            && self[0][1].cc_tol(other[0][1], tolerance)
            && self[0][2].cc_tol(other[0][2], tolerance)
            && self[1][0].cc_tol(other[1][0], tolerance)
            && self[1][1].cc_tol(other[1][1], tolerance)
            && self[1][2].cc_tol(other[1][2], tolerance)
            && self[2][0].cc_tol(other[2][0], tolerance)
            && self[2][1].cc_tol(other[2][1], tolerance)
            && self[2][2].cc_tol(other[2][2], tolerance)
    }
}

impl<S: Scalar> Coincidence<S> for Mat44<S> {
    fn cc_tol(self, other: Self, tolerance: f64) -> bool {
        self[0][0].cc_tol(other[0][0], tolerance)
            && self[0][1].cc_tol(other[0][1], tolerance)
            && self[0][2].cc_tol(other[0][2], tolerance)
            && self[0][3].cc_tol(other[0][3], tolerance)
            && self[1][0].cc_tol(other[1][0], tolerance)
            && self[1][1].cc_tol(other[1][1], tolerance)
            && self[1][2].cc_tol(other[1][2], tolerance)
            && self[1][3].cc_tol(other[1][3], tolerance)
            && self[2][0].cc_tol(other[2][0], tolerance)
            && self[2][1].cc_tol(other[2][1], tolerance)
            && self[2][2].cc_tol(other[2][2], tolerance)
            && self[2][3].cc_tol(other[2][3], tolerance)
            && self[3][0].cc_tol(other[3][0], tolerance)
            && self[3][1].cc_tol(other[3][1], tolerance)
            && self[3][2].cc_tol(other[3][2], tolerance)
            && self[3][3].cc_tol(other[3][3], tolerance)
    }
}

/*
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
    //fn coincident(&self, other: T, tolerance: f64) -> bool;

    fn cc(&self, other: T) -> bool {
        self.coincident(other, COINCIDENT_TOL)
    }

    fn cc_newton(&self, other: T) -> bool {
        self.coincident(other, NEWTON_TOL)
    }
}

impl Coincidence<f64> for f64 {
    fn cc(&self, other: f64) -> bool {
        within_tolerance_f64(self, other, COINCIDENT_TOL)
    }

    fn cc_newton(&self, other: f64) -> bool {
        within_tolerance_f64(self, other, NEWTON_TOL)
    }
}

impl<S: Scalar> Coincidence<Mat33<S>> for Mat33<S> {
    fn cc(&self, other: Mat33<S>) -> bool {
        for row in 0..3 {
            for col in 0..3 {
                if !within_tolerance_f64(self[row][col], other[row][col], tolerance) {
                    return false;
                }
            }
        }

        true
    }

    fn cc_newton(&self, other: Mat33<S>) -> bool {
        self.coincident(other, NEWTON_TOL)
    }
    /*
    fn coincident(&self, other: Mat33, tolerance: f64) -> bool {
        for row in 0..3 {
            for col in 0..3 {
                if !within_tolerance_f64(self[row][col], other[row][col], tolerance) {
                    return false;
                }
            }
        }

        true
    }
    */
}

impl Coincidence<Mat44> for Mat44 {
    fn coincident(&self, other: Mat44, tolerance: f64) -> bool {
        for row in 0..4 {
            for col in 0..4 {
                if !within_tolerance_f64(self[row][col], other[row][col], tolerance) {
                    return false;
                }
            }
        }

        true
    }
}

impl Coincidence<Vec2> for Vec2 {
    /// To be considered geometrically coincident,
    /// vectors are treated as points and those points
    /// must be separated by a distance near `0.0`
    fn coincident(&self, other: Vec2, tolerance: f64) -> bool {
        self.into_point().coincident(other.into_point(), tolerance)
    }
}

impl Coincidence<Vec3> for Vec3 {
    /// To be considered geometrically coincident,
    /// vectors are treated as points and those points
    /// must be separated by a distance near `0.0`
    fn coincident(&self, other: Vec3, tolerance: f64) -> bool {
        self.into_point().coincident(other.into_point(), tolerance)
    }
}

impl Coincidence<Point2> for Point2 {
    /// To be considered geometrically coincident,
    /// points must be separated by a distance near `0.0`
    fn coincident(&self, other: Point2, tolerance: f64) -> bool {
        within_tolerance_f64((*self - other).magnitude(), 0.0, tolerance)
    }
}

impl Coincidence<Point3> for Point3 {
    /// To be considered geometrically coincident,
    /// points must be separated by a distance near `0.0`
    fn coincident(&self, other: Point3, tolerance: f64) -> bool {
        within_tolerance_f64((*self - other).magnitude(), 0.0, tolerance)
    }
}

impl Coincidence<Angle> for Angle {
    fn coincident(&self, other: Angle, tolerance: f64) -> bool {
        within_tolerance_f64(self.radians(), other.radians(), tolerance)
    }
}

#[cfg(test)]
mod tests {
    use crate::math::{point2, vec2};

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
*/
