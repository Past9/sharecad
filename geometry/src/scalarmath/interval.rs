use super::{Float, Scalar};
use auto_ops::impl_op_ex;

#[derive(Copy, Clone, PartialEq)]
pub struct Interval(pub Float, pub Float);
impl Interval {
    pub const EMPTY: Self = Self(Float::NAN, Float::NAN);

    pub fn thin(val: Float) -> Self {
        Self(val, val)
    }

    pub fn is_empty(self) -> bool {
        self == Self::EMPTY
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

    pub fn rad(self) -> Float {
        (self.1 - self.0) / Float(2.0)
    }

    pub fn mid(self) -> Float {
        (self.1 + self.0) / Float(2.0)
    }

    pub fn contains_exact(self, val: Float) -> bool {
        self.0 <= val && self.1 >= val
    }

    pub fn contains_zero(self) -> bool {
        self.contains_exact(Float(0.0))
    }

    pub fn mig(self) -> Float {
        if self.contains_zero() {
            Float(0.0)
        } else {
            self.0.abs().min(self.1.abs())
        }
    }

    pub fn mag(self) -> Float {
        self.0.abs().max(self.1.abs())
    }

    pub fn hausdorff(self, rhs: Self) -> Float {
        (self.0 - rhs.0).abs().max(self.1 - rhs.1.abs())
    }

    pub fn round_out(self) -> Self {
        Self(self.0.prev(), self.1.next())
    }
}
impl Scalar for Interval {
    const E: Self = Self(Float(2.718281828459045), Float(2.7182818284590455));
    const FRAC_1_PI: Self = Self(Float(0.31830988618379064), Float(0.3183098861837907));
    const FRAC_1_SQRT_2: Self = Self(Float(0.7071067811865475), Float(0.7071067811865476));
    const FRAC_2_PI: Self = Self(Float(0.6366197723675813), Float(0.6366197723675814));
    const FRAC_2_SQRT_PI: Self = Self(Float(1.1283791670955126), Float(1.1283791670955128));
    const FRAC_PI_2: Self = Self(Float(1.5707963267948966), Float(1.5707963267948968));
    const FRAC_PI_3: Self = Self(Float(1.0471975511965976), Float(1.0471975511965979));
    const FRAC_PI_4: Self = Self(Float(0.7853981633974483), Float(0.7853981633974484));
    const FRAC_PI_6: Self = Self(Float(0.5235987755982988), Float(0.5235987755982989));
    const FRAC_PI_8: Self = Self(Float(0.39269908169872414), Float(0.3926990816987242));
    const LN_10: Self = Self(Float(2.3025850929940455), Float(2.302585092994046));
    const LN_2: Self = Self(Float(0.6931471805599453), Float(0.6931471805599454));
    const LOG10_2: Self = Self(Float(0.30102999566398114), Float(0.3010299956639812));
    const LOG10_E: Self = Self(Float(0.4342944819032518), Float(0.43429448190325187));
    const LOG2_10: Self = Self(Float(3.321928094887362), Float(3.3219280948873626));
    const LOG2_E: Self = Self(Float(1.4426950408889634), Float(1.4426950408889636));
    const PI: Self = Self(Float(3.141592653589793), Float(3.1415926535897936));
    const SQRT_2: Self = Self(Float(1.414213562373095), Float(1.4142135623730951));
    const TAU: Self = Self(Float(6.283185307179586), Float(6.283185307179587));

    fn powi(self, n: i32) -> Self {
        if n.is_positive() && n % 2 == 1 {
            Self(self.0.powi(n), self.1.powi(n))
        } else if n.is_positive() && n % 2 == 0 {
            Self(self.mig().powi(n), self.mag().powi(n))
        } else if n == 0 {
            Self(Float(1.0), Float(1.0))
        } else if n.is_negative() && !self.contains_zero() {
            Self(Float(1.0) / self.1, Float(1.0) / self.0)
                .powi(-n)
                .round_out()
        } else {
            unreachable!("({}).powi({}) is undefined", self, n);
        }
    }

    fn sqrt(self) -> Self {
        Self(self.0.sqrt(), self.1.sqrt()).round_out()
    }

    fn exp(self) -> Self {
        Self(self.0.exp(), self.1.exp()).round_out()
    }

    fn abs(self) -> Self {
        Self(self.mig(), self.mag())
    }

    fn atan(self) -> Self {
        Self(self.0.atan(), self.1.atan()).round_out()
    }

    fn sin(self) -> Self {
        if self.is_empty() {
            return Self::EMPTY;
        }

        let diff = Float(self.0 .0.rem_euclid(std::f64::consts::TAU) - self.0 .0);
        let norm = self + Self::thin(diff);
        let has_peak = norm.intersects(Self::FRAC_PI_2);
        let has_trough = norm.intersects(Self::FRAC_PI_2 * Self::thin(Float(3.0)));

        println!("self = {}", self);
        println!("diff = {}", diff);
        println!("norm = {}", norm);
        println!("has_peak = {}", has_peak);
        println!("has_trough = {}", has_trough);

        match (has_trough, has_peak) {
            (true, true) => Self(Float(-1.0), Float(1.0)),
            (true, false) => Self(Float(-1.0), self.0.sin().max(self.1.sin())),
            (false, true) => Self(self.0.sin().min(self.1.sin()), Float(1.0)),
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
