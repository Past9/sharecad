use gen_ops::gen_ops;
use std::ops::{Add, Div, Mul, Neg, Sub};

use crate::math::{vec3, vec4, Mat22, Mat33};

use super::{tolerance::COINCIDENT_TOL, vec2, Coincidence, Mat44, Vec2, Vec3, Vec4};

pub trait Scalar:
    std::fmt::Debug
    + std::fmt::Display
    + Sized
    + PartialEq
    + Clone
    + Copy
    + Neg<Output = Self>
    + Add<Self, Output = Self>
    + Sub<Self, Output = Self>
    + Mul<Self, Output = Self>
    + Div<Self, Output = Self>
    + From<f32>
    + Coincidence<Self>
    //
    + Add<Vec2<Self>, Output = Vec2<Self>>
    + Sub<Vec2<Self>, Output = Vec2<Self>>
    + Mul<Vec2<Self>, Output = Vec2<Self>>
    + Div<Vec2<Self>, Output = Vec2<Self>>
    //
    + Add<Vec3<Self>, Output = Vec3<Self>>
    + Sub<Vec3<Self>, Output = Vec3<Self>>
    + Mul<Vec3<Self>, Output = Vec3<Self>>
    + Div<Vec3<Self>, Output = Vec3<Self>>
    //
    + Add<Vec4<Self>, Output = Vec4<Self>>
    + Sub<Vec4<Self>, Output = Vec4<Self>>
    + Mul<Vec4<Self>, Output = Vec4<Self>>
    + Div<Vec4<Self>, Output = Vec4<Self>>
    //
    + Add<Mat22<Self>, Output = Mat22<Self>>
    + Sub<Mat22<Self>, Output = Mat22<Self>>
    + Mul<Mat22<Self>, Output = Mat22<Self>>
    + Div<Mat22<Self>, Output = Mat22<Self>>
    //
    + Add<Mat33<Self>, Output = Mat33<Self>>
    + Sub<Mat33<Self>, Output = Mat33<Self>>
    + Mul<Mat33<Self>, Output = Mat33<Self>>
    + Div<Mat33<Self>, Output = Mat33<Self>>
    //
    + Add<Mat44<Self>, Output = Mat44<Self>>
    + Sub<Mat44<Self>, Output = Mat44<Self>>
    + Mul<Mat44<Self>, Output = Mat44<Self>>
    + Div<Mat44<Self>, Output = Mat44<Self>>
{
    const ZERO: Self;
    const HALF: Self;
    const ONE: Self;
    const TWO: Self;
    const FOUR: Self;

    const PI: Self;
    const TAU: Self;
    const FRAC_PI_2: Self;
    const FRAC_PI_3: Self;
    const FRAC_PI_4: Self;
    const FRAC_PI_6: Self;
    const FRAC_PI_8: Self;
    const FRAC_1_PI: Self;
    const FRAC_2_PI: Self;
    const FRAC_2_SQRT_PI: Self;
    const SQRT_2: Self;
    const FRAC_1_SQRT_2: Self;
    const E: Self;
    const LOG2_10: Self;
    const LOG2_E: Self;
    const LOG10_2: Self;
    const LOG10_E: Self;
    const LN_2: Self;
    const LN_10: Self;

    fn as_f64(self) -> f64;
    fn as_f32(self) -> f32;
    fn exact(val: f64) -> Self;

    fn sin(self) -> Self;
    fn cos(self) -> Self;
    fn tan(self) -> Self;
    fn csc(self) -> Self;
    fn sec(self) -> Self;
    fn cot(self) -> Self;
    fn sin_cos(self) -> (Self, Self);

    fn recip(self) -> Self;
    fn abs(self) -> Self;
    fn powi(self, n: i32) -> Self;
    fn sqrt(self) -> Self;
    fn clamp(self, min: Self, max: Self) -> Self;

    fn lerp(self, to: Self, t: Self) -> Self {
        (Self::ONE - t) * self + t * to
    }

}

impl Scalar for f64 {
    const ZERO: Self = 0.0;
    const HALF: Self = 0.5;
    const ONE: Self = 1.0;
    const TWO: Self = 2.0;
    const FOUR: Self = 4.0;

    const PI: Self = std::f64::consts::PI;
    const TAU: Self = std::f64::consts::TAU;
    const FRAC_PI_2: Self = std::f64::consts::FRAC_PI_2;
    const FRAC_PI_3: Self = std::f64::consts::FRAC_PI_3;
    const FRAC_PI_4: Self = std::f64::consts::FRAC_PI_4;
    const FRAC_PI_6: Self = std::f64::consts::FRAC_PI_6;
    const FRAC_PI_8: Self = std::f64::consts::FRAC_PI_8;
    const FRAC_1_PI: Self = std::f64::consts::FRAC_1_PI;
    const FRAC_2_PI: Self = std::f64::consts::FRAC_2_PI;
    const FRAC_2_SQRT_PI: Self = std::f64::consts::FRAC_2_SQRT_PI;
    const SQRT_2: Self = std::f64::consts::SQRT_2;
    const FRAC_1_SQRT_2: Self = std::f64::consts::FRAC_1_SQRT_2;
    const E: Self = std::f64::consts::E;
    const LOG2_10: Self = std::f64::consts::LOG2_10;
    const LOG2_E: Self = std::f64::consts::LOG2_E;
    const LOG10_2: Self = std::f64::consts::LOG10_2;
    const LOG10_E: Self = std::f64::consts::LOG10_E;
    const LN_2: Self = std::f64::consts::LN_2;
    const LN_10: Self = std::f64::consts::LN_10;

    fn as_f64(self) -> f64 {
        self
    }

    fn as_f32(self) -> f32 {
        self as f32
    }

    fn exact(val: f64) -> Self {
        val
    }

    fn sin(self) -> Self {
        self.sin()
    }

    fn cos(self) -> Self {
        self.cos()
    }

    fn tan(self) -> Self {
        self.tan()
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
        self.sin_cos()
    }

    fn recip(self) -> Self {
        self.recip()
    }

    fn abs(self) -> Self {
        self.abs()
    }

    fn powi(self, n: i32) -> Self {
        self.powi(n)
    }

    fn sqrt(self) -> Self {
        self.sqrt()
    }

    fn clamp(self, min: Self, max: Self) -> Self {
        self.clamp(min, max)
    }
}

gen_ops!(
    types f64, Vec2<f64> => Vec2<f64>;
    for + call |l: &f64, r: &Vec2<f64>| {
        vec2(l + r.x, l + r.y)
    };
    for - call |l: &f64, r: &Vec2<f64>| {
        vec2(l - r.x, l - r.y)
    };
    for * call |l: &f64, r: &Vec2<f64>| {
        vec2(l * r.x, l * r.y)
    };
    for / call |l: &f64, r: &Vec2<f64>| {
        vec2(l / r.x, l / r.y)
    };
);

gen_ops!(
    types f64, Vec3<f64> => Vec3<f64>;
    for + call |l: &f64, r: &Vec3<f64>| {
        vec3(l + r.x, l + r.y, l + r.z)
    };
    for - call |l: &f64, r: &Vec3<f64>| {
        vec3(l - r.x, l - r.y, l - r.z)
    };
    for * call |l: &f64, r: &Vec3<f64>| {
        vec3(l * r.x, l * r.y, l * r.z)
    };
    for / call |l: &f64, r: &Vec3<f64>| {
        vec3(l / r.x, l / r.y, l / r.z)
    };
);

gen_ops!(
    types f64, Vec4<f64> => Vec4<f64>;
    for + call |l: &f64, r: &Vec4<f64>| {
        vec4(l + r.x, l + r.y, l + r.z, l + r.w)
    };
    for - call |l: &f64, r: &Vec4<f64>| {
        vec4(l - r.x, l - r.y, l - r.z, l - r.w)
    };
    for * call |l: &f64, r: &Vec4<f64>| {
        vec4(l * r.x, l * r.y, l * r.z, l * r.w)
    };
    for / call |l: &f64, r: &Vec4<f64>| {
        vec4(l / r.x, l / r.y, l / r.z, l / r.w)
    };
);

gen_ops!(
    types f64, Mat22<f64> => Mat22<f64>;
    for + call |l: &f64, r: &Mat22<f64>| {
        Mat22([
            [l + r[0][0], l + r[0][1]],
            [l + r[1][0], l + r[1][1]],
        ])
    };
    for - call |l: &f64, r: &Mat22<f64>| {
        Mat22([
            [l - r[0][0], l - r[0][1]],
            [l - r[1][0], l - r[1][1]],
        ])
    };
    for * call |l: &f64, r: &Mat22<f64>| {
        Mat22([
            [l * r[0][0], l * r[0][1]],
            [l * r[1][0], l * r[1][1]],
        ])
    };
    for / call |l: &f64, r: &Mat22<f64>| {
        Mat22([
            [l / r[0][0], l / r[0][1]],
            [l / r[1][0], l / r[1][1]],
        ])
    };
);

gen_ops!(
    types f64, Mat33<f64> => Mat33<f64>;
    for + call |l: &f64, r: &Mat33<f64>| {
        Mat33([
            [l + r[0][0], l + r[0][1], l + r[0][2]],
            [l + r[1][0], l + r[1][1], l + r[1][2]],
            [l + r[2][0], l + r[2][1], l + r[2][2]],
        ])
    };
    for - call |l: &f64, r: &Mat33<f64>| {
        Mat33([
            [l - r[0][0], l - r[0][1], l - r[0][2]],
            [l - r[1][0], l - r[1][1], l - r[1][2]],
            [l - r[2][0], l - r[2][1], l - r[2][2]],
        ])
    };
    for * call |l: &f64, r: &Mat33<f64>| {
        Mat33([
            [l * r[0][0], l * r[0][1], l * r[0][2]],
            [l * r[1][0], l * r[1][1], l * r[1][2]],
            [l * r[2][0], l * r[2][1], l * r[2][2]],
        ])
    };
    for / call |l: &f64, r: &Mat33<f64>| {
        Mat33([
            [l / r[0][0], l / r[0][1], l / r[0][2]],
            [l / r[1][0], l / r[1][1], l / r[1][2]],
            [l / r[2][0], l / r[2][1], l / r[2][2]],
        ])
    };
);

gen_ops!(
    types f64, Mat44<f64> => Mat44<f64>;
    for + call |l: &f64, r: &Mat44<f64>| {
        Mat44([
            [l + r[0][0], l + r[0][1], l + r[0][2], l + r[0][3]],
            [l + r[1][0], l + r[1][1], l + r[1][2], l + r[1][3]],
            [l + r[2][0], l + r[2][1], l + r[2][2], l + r[2][3]],
            [l + r[3][0], l + r[3][1], l + r[3][2], l + r[3][3]],
        ])
    };
    for - call |l: &f64, r: &Mat44<f64>| {
        Mat44([
            [l - r[0][0], l - r[0][1], l - r[0][2], l - r[0][3]],
            [l - r[1][0], l - r[1][1], l - r[1][2], l - r[1][3]],
            [l - r[2][0], l - r[2][1], l - r[2][2], l - r[2][3]],
            [l - r[3][0], l - r[3][1], l - r[3][2], l - r[3][3]],
        ])
    };
    for * call |l: &f64, r: &Mat44<f64>| {
        Mat44([
            [l * r[0][0], l * r[0][1], l * r[0][2], l * r[0][3]],
            [l * r[1][0], l * r[1][1], l * r[1][2], l * r[1][3]],
            [l * r[2][0], l * r[2][1], l * r[2][2], l * r[2][3]],
            [l * r[3][0], l * r[3][1], l * r[3][2], l * r[3][3]],
        ])
    };
    for / call |l: &f64, r: &Mat44<f64>| {
        Mat44([
            [l / r[0][0], l / r[0][1], l / r[0][2], l / r[0][3]],
            [l / r[1][0], l / r[1][1], l / r[1][2], l / r[1][3]],
            [l / r[2][0], l / r[2][1], l / r[2][2], l / r[2][3]],
            [l / r[3][0], l / r[3][1], l / r[3][2], l / r[3][3]],
        ])
    };
);

/*
impl std::ops::Add<Vec2<f64>> for f64 {
    type Output = Vec2<f64>;

    fn add(self, rhs: Vec2<f64>) -> Self::Output {
        vec2(self * rhs.x, self * rhs.y)
    }
}
impl std::ops::Sub<Vec2<f64>> for f64 {
    type Output = Vec2<f64>;

    fn add(self, rhs: Vec2<f64>) -> Self::Output {
        vec2(self * rhs.x, self * rhs.y)
    }
}
impl std::ops::Mul<Vec2<f64>> for f64 {
    type Output = Vec2<f64>;

    fn add(self, rhs: Vec2<f64>) -> Self::Output {
        vec2(self * rhs.x, self * rhs.y)
    }
}
impl std::ops::Div<Vec2<f64>> for f64 {
    type Output = Vec2<f64>;

    fn add(self, rhs: Vec2<f64>) -> Self::Output {
        vec2(self * rhs.x, self * rhs.y)
    }
}
 */

//+ Add<Vec2<Self>, Output = Vec2<Self>>
