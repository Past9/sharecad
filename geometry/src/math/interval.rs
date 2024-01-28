use crate::math::{vec2, vec3, vec4};

use super::{
    tolerance::COINCIDENT_TOL, Coincidence, Mat22, Mat33, Mat44, Scalar, Vec2, Vec3, Vec4,
};
use auto_ops::impl_op_ex;
use float_cmp::Ulps;
use gen_ops::gen_ops;
use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Copy, Clone, PartialEq)]
pub struct Interval(pub f64, pub f64);
impl Interval {
    //pub const EMPTY: Self = Self(f64::NAN, f64::NAN);
    pub const EMPTY: Self = Self(f64::INFINITY, f64::NEG_INFINITY);

    pub const fn thin(val: f64) -> Self {
        Self(val, val)
    }

    pub fn is_empty(self) -> bool {
        //self == Self::EMPTY
        self.0 > self.1
    }

    pub fn is_subset_of(self, rhs: Self) -> bool {
        rhs.0 <= self.0 && self.1 <= rhs.1
    }

    pub fn is_proper_subset_of(self, rhs: Self) -> bool {
        self.is_subset_of(rhs) && self != rhs
    }

    pub fn is_strict_subset_of(self, rhs: Self) -> bool {
        rhs.0 < self.0 && self.1 < rhs.1
    }

    pub fn intersection(self, rhs: Self) -> Self {
        if !self.intersects(rhs) {
            Self::EMPTY
        } else {
            Self(self.0.max(rhs.0), self.1.min(rhs.1))
        }
    }

    pub fn intersects(self, rhs: Self) -> bool {
        if self.is_empty() || rhs.is_empty() {
            return false;
        }

        !(self.1 < rhs.0 || rhs.1 < self.0)
    }

    pub fn rad(self) -> f64 {
        (self.1 - self.0) / 2.0
    }

    pub fn mid(self) -> f64 {
        (self.1 + self.0) / 2.0
    }

    pub fn contains_exact(self, val: f64) -> bool {
        self.0 <= val && self.1 >= val
    }

    pub fn contains_zero(self) -> bool {
        self.contains_exact(0.0)
    }

    pub fn mig(self) -> f64 {
        if self.contains_zero() {
            0.0
        } else {
            self.0.abs().min(self.1.abs())
        }
    }

    pub fn mag(self) -> f64 {
        self.0.abs().max(self.1.abs())
    }

    pub fn hausdorff(self, rhs: Self) -> f64 {
        (self.0 - rhs.0).abs().max(self.1 - rhs.1.abs())
    }

    pub fn round_out(self) -> Self {
        Self(self.0.prev(), self.1.next())
    }

    pub fn split_on_zero(&self) -> Vec<Self> {
        if self.0 < 0.0 && self.1 > 0.0 {
            vec![Self(self.0, (-0.0).prev()), Self(0.0.next(), self.1)]
        } else {
            vec![*self]
        }
        .into_iter()
        .filter(|ivl| !ivl.is_empty())
        .collect()
    }
}
impl From<(f64, f64)> for Interval {
    fn from(value: (f64, f64)) -> Self {
        Self(value.0, value.1)
    }
}
impl Scalar for Interval {
    const ZERO: Self = Self::thin(0.0);
    const HALF: Self = Self::thin(0.5);
    const ONE: Self = Self::thin(1.0);
    const TWO: Self = Self::thin(2.0);
    const FOUR: Self = Self::thin(4.0);

    const E: Self = Self(2.718281828459045, 2.7182818284590455);
    const FRAC_1_PI: Self = Self(0.31830988618379064, 0.3183098861837907);
    const FRAC_1_SQRT_2: Self = Self(0.7071067811865475, 0.7071067811865476);
    const FRAC_2_PI: Self = Self(0.6366197723675813, 0.6366197723675814);
    const FRAC_2_SQRT_PI: Self = Self(1.1283791670955126, 1.1283791670955128);
    const FRAC_PI_2: Self = Self(1.5707963267948966, 1.5707963267948968);
    const FRAC_PI_3: Self = Self(1.0471975511965976, 1.0471975511965979);
    const FRAC_PI_4: Self = Self(0.7853981633974483, 0.7853981633974484);
    const FRAC_PI_6: Self = Self(0.5235987755982988, 0.5235987755982989);
    const FRAC_PI_8: Self = Self(0.39269908169872414, 0.3926990816987242);
    const LN_10: Self = Self(2.3025850929940455, 2.302585092994046);
    const LN_2: Self = Self(0.6931471805599453, 0.6931471805599454);
    const LOG10_2: Self = Self(0.30102999566398114, 0.3010299956639812);
    const LOG10_E: Self = Self(0.4342944819032518, 0.43429448190325187);
    const LOG2_10: Self = Self(3.321928094887362, 3.3219280948873626);
    const LOG2_E: Self = Self(1.4426950408889634, 1.4426950408889636);
    const PI: Self = Self(3.141592653589793, 3.1415926535897936);
    const SQRT_2: Self = Self(1.414213562373095, 1.4142135623730951);
    const TAU: Self = Self(6.283185307179586, 6.283185307179587);

    fn powi(self, n: i32) -> Self {
        if n.is_positive() && n % 2 == 1 {
            Self(self.0.powi(n), self.1.powi(n))
        } else if n.is_positive() && n % 2 == 0 {
            Self(self.mig().powi(n), self.mag().powi(n))
        } else if n == 0 {
            Self(1.0, 1.0)
        } else if n.is_negative() && !self.contains_zero() {
            Self(1.0 / self.1, 1.0 / self.0).powi(-n).round_out()
        } else {
            unreachable!("({}).powi({}) is undefined", self, n);
        }
    }

    fn sqrt(self) -> Self {
        Self(self.0.sqrt(), self.1.sqrt()).round_out()
    }

    fn abs(self) -> Self {
        Self(self.mig(), self.mag())
    }

    fn sin(self) -> Self {
        if self.is_empty() {
            return Self::EMPTY;
        }

        let diff = self.0.rem_euclid(std::f64::consts::TAU) - self.0;
        let norm = self + Self::thin(diff);
        let has_peak = norm.intersects(Self::FRAC_PI_2);
        let has_trough = norm.intersects(Self::FRAC_PI_2 * Self::thin(3.0));

        /*
        println!("self = {}", self);
        println!("diff = {}", diff);
        println!("norm = {}", norm);
        println!("has_peak = {}", has_peak);
        println!("has_trough = {}", has_trough);
         */

        match (has_trough, has_peak) {
            (true, true) => Self(-1.0, 1.0),
            (true, false) => Self(-1.0, self.0.sin().max(self.1.sin())),
            (false, true) => Self(self.0.sin().min(self.1.sin()), 1.0),
            (false, false) => {
                let sin_l = self.0.sin();
                let sin_h = self.1.sin();
                Self(sin_l.min(sin_h), sin_l.max(sin_h))
            }
        }
    }

    fn cos(self) -> Self {
        (self + Self::FRAC_PI_2).sin()
    }

    fn tan(self) -> Self {
        self.sin() / self.cos()
    }

    fn as_f64(self) -> f64 {
        todo!()
    }

    fn as_f32(self) -> f32 {
        todo!()
    }

    fn exact(val: f64) -> Self {
        todo!()
    }

    fn csc(self) -> Self {
        Self::ONE / self.sin()
    }

    fn sec(self) -> Self {
        Self::ONE / self.cos()
    }

    fn cot(self) -> Self {
        Self::ONE / self.tan()
    }

    fn sin_cos(self) -> (Self, Self) {
        (self.sin(), self.cos())
    }

    fn recip(self) -> Self {
        todo!()
    }

    fn clamp(self, min: Self, max: Self) -> Self {
        todo!()
    }
}
impl From<f32> for Interval {
    fn from(value: f32) -> Self {
        Self::thin(value as f64)
    }
}
impl std::fmt::Display for Interval {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("[{}…{}]", self.0, self.1))
    }
}
impl std::fmt::Debug for Interval {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self, f)
    }
}

gen_ops!(
    types Interval => Interval;
    for - call |v: &Interval| {
        if v.is_empty() {
            return Interval::EMPTY;
        }
        Interval(-v.1, -v.0)
    };
);

gen_ops!(
    types Interval, Interval => Interval;
    for + call |l: &Interval, r: &Interval| {
        if l.is_empty() || r.is_empty() {
            return Interval::EMPTY;
        }
        Interval(l.0 + r.0, l.1 + r.1).round_out()
    };
    for - call |l: &Interval, r: &Interval| {
        if l.is_empty() || r.is_empty() {
            return Interval::EMPTY;
        }
        Interval(l.0 - r.1, l.1 - r.0).round_out()
    };
    for * call |l: &Interval, r: &Interval| {
        if l.is_empty() || r.is_empty() {
            return Interval::EMPTY;
        }

        let l0r0 = l.0 * r.0;
        let l1r0 = l.1 * r.0;
        let l0r1 = l.0 * r.1;
        let l1r1 = l.1 * r.1;

        Interval(
            l0r0.min(l1r0).min(l0r1).min(l1r1),
            l0r0.max(l1r0).max(l0r1).max(l1r1),
        )
        .round_out()
    };
    for / call |l: &Interval, r: &Interval| {
        if l.is_empty() || r.is_empty() {
            return Interval::EMPTY;
        }

        if r.contains_zero() {
            //panic!("denominator of {}/{} straddles zero", l, r);
        }

        let l0r0 = l.0 / r.0;
        let l1r0 = l.1 / r.0;
        let l0r1 = l.0 / r.1;
        let l1r1 = l.1 / r.1;

        Interval(
            l0r0.min(l1r0).min(l0r1).min(l1r1),
            l0r0.max(l1r0).max(l0r1).max(l1r1),
        )
        .round_out()
    };
);

gen_ops!(
    types Interval, Vec2<Interval> => Vec2<Interval>;
    for + call |l: &Interval, r: &Vec2<Interval>| {
        vec2(*l + r.x, *l + r.y)
    };
    for - call |l: &Interval, r: &Vec2<Interval>| {
        vec2(*l - r.x, *l - r.y)
    };
    for * call |l: &Interval, r: &Vec2<Interval>| {
        vec2(*l * r.x, *l * r.y)
    };
    for / call |l: &Interval, r: &Vec2<Interval>| {
        vec2(*l / r.x, *l / r.y)
    };
);

gen_ops!(
    types Interval, Vec3<Interval> => Vec3<Interval>;
    for + call |l: &Interval, r: &Vec3<Interval>| {
        vec3(*l + r.x, *l + r.y, *l + r.z)
    };
    for - call |l: &Interval, r: &Vec3<Interval>| {
        vec3(*l - r.x, *l - r.y, *l - r.z)
    };
    for * call |l: &Interval, r: &Vec3<Interval>| {
        vec3(*l * r.x, *l * r.y, *l * r.z)
    };
    for / call |l: &Interval, r: &Vec3<Interval>| {
        vec3(*l / r.x, *l / r.y, *l / r.z)
    };
);

gen_ops!(
    types Interval, Vec4<Interval> => Vec4<Interval>;
    for + call |l: &Interval, r: &Vec4<Interval>| {
        vec4(*l + r.x, *l + r.y, *l + r.z, *l + r.w)
    };
    for - call |l: &Interval, r: &Vec4<Interval>| {
        vec4(*l - r.x, *l - r.y, *l - r.z, *l - r.w)
    };
    for * call |l: &Interval, r: &Vec4<Interval>| {
        vec4(*l * r.x, *l * r.y, *l * r.z, *l * r.w)
    };
    for / call |l: &Interval, r: &Vec4<Interval>| {
        vec4(*l / r.x, *l / r.y, *l / r.z, *l / r.w)
    };
);

gen_ops!(
    types Interval, Mat22<Interval> => Mat22<Interval>;
    for + call |l: &Interval, r: &Mat22<Interval>| {
        Mat22([
            [*l + r[0][0], *l + r[0][1]],
            [*l + r[1][0], *l + r[1][1]],
        ])
    };
    for - call |l: &Interval, r: &Mat22<Interval>| {
        Mat22([
            [*l - r[0][0], *l - r[0][1]],
            [*l - r[1][0], *l - r[1][1]],
        ])
    };
    for * call |l: &Interval, r: &Mat22<Interval>| {
        Mat22([
            [*l * r[0][0], *l * r[0][1]],
            [*l * r[1][0], *l * r[1][1]],
        ])
    };
    for / call |l: &Interval, r: &Mat22<Interval>| {
        Mat22([
            [*l / r[0][0], *l / r[0][1]],
            [*l / r[1][0], *l / r[1][1]],
        ])
    };
);

gen_ops!(
    types Interval, Mat33<Interval> => Mat33<Interval>;
    for + call |l: &Interval, r: &Mat33<Interval>| {
        Mat33([
            [*l + r[0][0], *l + r[0][1], *l + r[0][2]],
            [*l + r[1][0], *l + r[1][1], *l + r[1][2]],
            [*l + r[2][0], *l + r[2][1], *l + r[2][2]],
        ])
    };
    for - call |l: &Interval, r: &Mat33<Interval>| {
        Mat33([
            [*l - r[0][0], *l - r[0][1], *l - r[0][2]],
            [*l - r[1][0], *l - r[1][1], *l - r[1][2]],
            [*l - r[2][0], *l - r[2][1], *l - r[2][2]],
        ])
    };
    for * call |l: &Interval, r: &Mat33<Interval>| {
        Mat33([
            [*l * r[0][0], *l * r[0][1], *l * r[0][2]],
            [*l * r[1][0], *l * r[1][1], *l * r[1][2]],
            [*l * r[2][0], *l * r[2][1], *l * r[2][2]],
        ])
    };
    for / call |l: &Interval, r: &Mat33<Interval>| {
        Mat33([
            [*l / r[0][0], *l / r[0][1], *l / r[0][2]],
            [*l / r[1][0], *l / r[1][1], *l / r[1][2]],
            [*l / r[2][0], *l / r[2][1], *l / r[2][2]],
        ])
    };
);

gen_ops!(
    types Interval, Mat44<Interval> => Mat44<Interval>;
    for + call |l: &Interval, r: &Mat44<Interval>| {
        Mat44([
            [*l + r[0][0], *l + r[0][1], *l + r[0][2], *l + r[0][3]],
            [*l + r[1][0], *l + r[1][1], *l + r[1][2], *l + r[1][3]],
            [*l + r[2][0], *l + r[2][1], *l + r[2][2], *l + r[2][3]],
            [*l + r[3][0], *l + r[3][1], *l + r[3][2], *l + r[3][3]],
        ])
    };
    for - call |l: &Interval, r: &Mat44<Interval>| {
        Mat44([
            [*l - r[0][0], *l - r[0][1], *l - r[0][2], *l - r[0][3]],
            [*l - r[1][0], *l - r[1][1], *l - r[1][2], *l - r[1][3]],
            [*l - r[2][0], *l - r[2][1], *l - r[2][2], *l - r[2][3]],
            [*l - r[3][0], *l - r[3][1], *l - r[3][2], *l - r[3][3]],
        ])
    };
    for * call |l: &Interval, r: &Mat44<Interval>| {
        Mat44([
            [*l * r[0][0], *l * r[0][1], *l * r[0][2], *l * r[0][3]],
            [*l * r[1][0], *l * r[1][1], *l * r[1][2], *l * r[1][3]],
            [*l * r[2][0], *l * r[2][1], *l * r[2][2], *l * r[2][3]],
            [*l * r[3][0], *l * r[3][1], *l * r[3][2], *l * r[3][3]],
        ])
    };
    for / call |l: &Interval, r: &Mat44<Interval>| {
        Mat44([
            [*l / r[0][0], *l / r[0][1], *l / r[0][2], *l / r[0][3]],
            [*l / r[1][0], *l / r[1][1], *l / r[1][2], *l / r[1][3]],
            [*l / r[2][0], *l / r[2][1], *l / r[2][2], *l / r[2][3]],
            [*l / r[3][0], *l / r[3][1], *l / r[3][2], *l / r[3][3]],
        ])
    };
);

/*
/// Checks whether the absolute value of the difference between `a` and `b`
/// is less than or equal to `tolerance`
pub fn within_tolerance_interval(a: Interval, b: Interval, tolerance: f64) -> bool {
    (a.mid() - b.mid()).abs() <= tolerance
}

impl Coincidence<Interval> for Interval {
    fn cc_tol(self, other: Self, tolerance: f64) -> bool {
        (a.mid() - b.mid()).abs() <= tolerance
    }

    fn cc(self, other: Self) -> bool {
        within_tolerance_interval(self, other, COINCIDENT_TOL)
    }
}

impl Coincidence<Interval> for Vec2<Interval> {
    fn cc(self, other: Self) -> bool {
        within_tolerance_interval(self.x, other.x, COINCIDENT_TOL)
            && within_tolerance_interval(self.y, other.y, COINCIDENT_TOL)
    }
}

impl Coincidence<Interval> for Vec3<Interval> {
    fn cc(self, other: Self) -> bool {
        within_tolerance_interval(self.x, other.x, COINCIDENT_TOL)
            && within_tolerance_interval(self.y, other.y, COINCIDENT_TOL)
            && within_tolerance_interval(self.z, other.z, COINCIDENT_TOL)
    }
}

impl Coincidence<Interval> for Vec4<Interval> {
    fn cc(self, other: Self) -> bool {
        within_tolerance_interval(self.x, other.x, COINCIDENT_TOL)
            && within_tolerance_interval(self.y, other.y, COINCIDENT_TOL)
            && within_tolerance_interval(self.z, other.z, COINCIDENT_TOL)
            && within_tolerance_interval(self.w, other.w, COINCIDENT_TOL)
    }
}

impl Coincidence<Interval> for Mat22<Interval> {
    fn cc(self, other: Self) -> bool {
        within_tolerance_interval(self[0][0], other[0][0], COINCIDENT_TOL)
            && within_tolerance_interval(self[0][1], other[0][1], COINCIDENT_TOL)
            && within_tolerance_interval(self[1][0], other[1][0], COINCIDENT_TOL)
            && within_tolerance_interval(self[1][1], other[1][1], COINCIDENT_TOL)
    }
}

impl Coincidence<Interval> for Mat33<Interval> {
    fn cc(self, other: Self) -> bool {
        within_tolerance_interval(self[0][0], other[0][0], COINCIDENT_TOL)
            && within_tolerance_interval(self[0][1], other[0][1], COINCIDENT_TOL)
            && within_tolerance_interval(self[0][2], other[0][2], COINCIDENT_TOL)
            && within_tolerance_interval(self[1][0], other[1][0], COINCIDENT_TOL)
            && within_tolerance_interval(self[1][1], other[1][1], COINCIDENT_TOL)
            && within_tolerance_interval(self[1][2], other[1][2], COINCIDENT_TOL)
            && within_tolerance_interval(self[2][0], other[2][0], COINCIDENT_TOL)
            && within_tolerance_interval(self[2][1], other[2][1], COINCIDENT_TOL)
            && within_tolerance_interval(self[2][2], other[2][2], COINCIDENT_TOL)
    }
}

impl Coincidence<Interval> for Mat44<Interval> {
    fn cc(self, other: Self) -> bool {
        within_tolerance_interval(self[0][0], other[0][0], COINCIDENT_TOL)
            && within_tolerance_interval(self[0][1], other[0][1], COINCIDENT_TOL)
            && within_tolerance_interval(self[0][2], other[0][2], COINCIDENT_TOL)
            && within_tolerance_interval(self[0][3], other[0][3], COINCIDENT_TOL)
            && within_tolerance_interval(self[1][0], other[1][0], COINCIDENT_TOL)
            && within_tolerance_interval(self[1][1], other[1][1], COINCIDENT_TOL)
            && within_tolerance_interval(self[1][2], other[1][2], COINCIDENT_TOL)
            && within_tolerance_interval(self[1][3], other[1][3], COINCIDENT_TOL)
            && within_tolerance_interval(self[2][0], other[2][0], COINCIDENT_TOL)
            && within_tolerance_interval(self[2][1], other[2][1], COINCIDENT_TOL)
            && within_tolerance_interval(self[2][2], other[2][2], COINCIDENT_TOL)
            && within_tolerance_interval(self[2][3], other[2][3], COINCIDENT_TOL)
            && within_tolerance_interval(self[3][0], other[3][0], COINCIDENT_TOL)
            && within_tolerance_interval(self[3][1], other[3][1], COINCIDENT_TOL)
            && within_tolerance_interval(self[3][2], other[3][2], COINCIDENT_TOL)
            && within_tolerance_interval(self[3][3], other[3][3], COINCIDENT_TOL)
    }
}
 */

/*
impl_op_ex!(-|i: &Interval| -> Interval {
    if i.is_empty() {
        return Interval::EMPTY;
    }
    Interval(-i.1, -i.0)
});

impl_op_ex!(+|l: &Interval, r: &Interval| -> Interval {
    if l.is_empty() || r.is_empty() {
        return Interval::EMPTY;
    }
    Interval(l.0 + r.0, l.1 + r.1).round_out()
});

impl_op_ex!(-|l: &Interval, r: &Interval| -> Interval {
    if l.is_empty() || r.is_empty() {
        return Interval::EMPTY;
    }

    Interval(l.0 - r.1, l.1 - r.0).round_out()
});

impl_op_ex!(*|l: &Interval, r: &Interval| -> Interval {
    if l.is_empty() || r.is_empty() {
        return Interval::EMPTY;
    }

    let l0r0 = l.0 * r.0;
    let l1r0 = l.1 * r.0;
    let l0r1 = l.0 * r.1;
    let l1r1 = l.1 * r.1;

    Interval(
        l0r0.min(l1r0).min(l0r1).min(l1r1),
        l0r0.max(l1r0).max(l0r1).max(l1r1),
    )
    .round_out()
});

impl_op_ex!(/|l: &Interval, r: &Interval| -> Interval {
    if l.is_empty() || r.is_empty() {
        return Interval::EMPTY;
    }

    if r.contains_zero() {
        panic!("denominator of {}/{} straddles zero", l, r);
    }

    let l0r0 = l.0 / r.0;
    let l1r0 = l.1 / r.0;
    let l0r1 = l.0 / r.1;
    let l1r1 = l.1 / r.1;

    Interval(
        l0r0.min(l1r0).min(l0r1).min(l1r1),
        l0r0.max(l1r0).max(l0r1).max(l1r1),
    )
    .round_out()
});
 */

/*
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sin() {
        assert_eq!(
            Interval(Float::FRAC_1_SQRT_2, Float(1.0)),
            Interval(Float::FRAC_PI_4, Float::FRAC_PI_4 * Float(3.0)).sin()
        );
    }

    #[test]
    fn cos() {
        assert_eq!(
            Interval(
                (-Float::FRAC_1_SQRT_2).prev(),
                Float::FRAC_1_SQRT_2.next().next().next()
            ),
            Interval(Float::FRAC_PI_4, Float::FRAC_PI_4 * Float(3.0)).cos()
        );
        assert_eq!(
            Interval(Float(-1.0), Float(1.0),),
            Interval(-Float::FRAC_PI_4, Float::FRAC_PI_2 * Float(3.0)).cos()
        );
    }

    #[test]
    fn tan() {
        assert_eq!(
            Interval(
                Float(-1.0).prev().prev().prev(),
                Float(1.0).next().next().next(),
            ),
            Interval(-Float::FRAC_PI_4, Float::FRAC_PI_4).tan()
        );
        assert_eq!(
            Interval(Float(-0.5463024898437907), Float(0.5463024898437907)),
            Interval(Float(-0.5), Float(0.5)).tan()
        );
        assert_eq!(
            Interval(Float(0.0), Float(0.5463024898437907)),
            Interval(Float(0.0), Float(0.5)).tan()
        );
        assert_eq!(
            Interval(Float(-0.5463024898437907), Float(0.0).next()),
            Interval(Float(-0.5), Float(0.0)).tan()
        );
    }

    #[test]
    fn test_func() {
        let x = Interval(Float(0.1), Float(1.0));

        println!("{}", x);
        println!("{}", Interval::PI * x);
        println!("{}", (Interval::PI * x).sin());
        println!("{}", my_func(x));
    }

    fn my_func<S: Scalar>(x: S) -> S {
        (S::PI * x).sin() / x
    }
}
*/
