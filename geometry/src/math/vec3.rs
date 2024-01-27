use super::{Mat33, Mat44, Scalar};
use gen_ops::gen_ops;

pub fn vec3<S: Scalar>(x: S, y: S, z: S) -> Vec3<S> {
    Vec3::new(x, y, z)
}

pub fn vec3_f32s<S: Scalar>(x: f32, y: f32, z: f32) -> Vec3<S> {
    Vec3::new(x.into(), y.into(), z.into())
}

#[derive(Copy, Clone)]
pub struct Vec3<S: Scalar> {
    pub x: S,
    pub y: S,
    pub z: S,
}
impl<S: Scalar> Vec3<S> {
    pub const ONES: Self = Self {
        x: S::ONE,
        y: S::ONE,
        z: S::ONE,
    };
    pub const ZERO: Self = Self {
        x: S::ZERO,
        y: S::ZERO,
        z: S::ZERO,
    };
    pub const UNIT_X: Self = Self {
        x: S::ONE,
        y: S::ZERO,
        z: S::ZERO,
    };
    pub const UNIT_Y: Self = Self {
        x: S::ZERO,
        y: S::ONE,
        z: S::ZERO,
    };
    pub const UNIT_Z: Self = Self {
        x: S::ZERO,
        y: S::ZERO,
        z: S::ONE,
    };

    /*
    pub fn is_zero(&self) -> bool {
        self.x == 0.0 && self.y == 0.0 && self.z == 0.0
    }
     */

    pub fn new(x: S, y: S, z: S) -> Self {
        Self { x, y, z }
    }

    pub fn magnitude2(self) -> S {
        self.dot(self)
    }

    pub fn magnitude(self) -> S {
        self.magnitude2().sqrt()
    }

    pub fn transform(&self, m: Mat44<S>) -> Self {
        let x = (self.x * m[0][0]) + (self.y * m[0][1]) + (self.z * m[0][2]) + m[0][3];
        let y = (self.x * m[1][0]) + (self.y * m[1][1]) + (self.z * m[1][2]) + m[1][3];
        let z = (self.x * m[2][0]) + (self.y * m[2][1]) + (self.z * m[2][2]) + m[2][3];
        let w = (self.x * m[3][0]) + (self.y * m[3][1]) + (self.z * m[3][2]) + m[3][3];
        Self {
            x: x / w,
            y: y / w,
            z: z / w,
        }
    }

    pub fn dot(self, other: Self) -> S {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(self, other: Self) -> Self {
        Self {
            x: (self.y * other.z) - (self.z * other.y),
            y: (self.z * other.x) - (self.x * other.z),
            z: (self.x * other.y) - (self.y * other.x),
        }
    }

    pub fn normalize(self) -> Self {
        self / self.magnitude()
    }

    pub fn to_f64s(self) -> [f64; 3] {
        [self.x.as_f64(), self.y.as_f64(), self.z.as_f64()]
    }

    pub fn to_f32s(self) -> [f32; 3] {
        [self.x.as_f32(), self.y.as_f32(), self.z.as_f32()]
    }

    pub fn powi(self, n: i32) -> Self {
        Self {
            x: self.x.powi(n),
            y: self.y.powi(n),
            z: self.z.powi(n),
        }
    }

    pub fn sum(self) -> S {
        self.x + self.y + self.z
    }

    pub fn lerp(self, other: Self, t: S) -> Self {
        (Self::ONES - t) * self + t * other
    }

    /// Returns the first derivative of `self.normalize()`, given the first
    /// derivative of `self`.
    pub fn norm_der1(self, der1: Self) -> Self {
        let g = self;
        let g_p = der1;
        let g_mag = g.magnitude();
        let f = g / g_mag;

        let g_p_over_g_mag = g_p / g_mag;

        g_p_over_g_mag - f.dot(g_p_over_g_mag) * f
    }

    /// Returns the second derivative of `self.normalize()`, given the first
    /// two derivatives of `self`
    pub fn norm_der2(self, der1: Self, der2: Self) -> Self {
        let g = self;
        let g_p = der1;
        let g_pp = der2;
        let g_mag = g.magnitude();
        let g_p_over_g_mag = g_p / g_mag;
        let g_pp_over_g_mag = g_pp / g_mag;
        let f = g / g_mag;
        let f_p = self.norm_der1(der1);

        g_pp_over_g_mag
            - S::TWO * (f.dot(g_p_over_g_mag)) * f_p
            - (f.dot(g_pp_over_g_mag) + f_p.dot(g_p_over_g_mag)) * f
    }
}
impl<S: Scalar> Default for Vec3<S> {
    fn default() -> Self {
        Self::ZERO
    }
}
impl<S: Scalar> From<[f64; 3]> for Vec3<S> {
    fn from(floats: [f64; 3]) -> Self {
        Self {
            x: S::exact(floats[0]),
            y: S::exact(floats[1]),
            z: S::exact(floats[2]),
        }
    }
}
impl<S: Scalar> From<[f32; 3]> for Vec3<S> {
    fn from(floats: [f32; 3]) -> Self {
        Self {
            x: S::exact(floats[0] as f64),
            y: S::exact(floats[1] as f64),
            z: S::exact(floats[2] as f64),
        }
    }
}
impl<S: Scalar> std::fmt::Display for Vec3<S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("[{}, {}, {}]", self.x, self.y, self.z))
    }
}
impl<S: Scalar> std::fmt::Debug for Vec3<S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("[{}, {}, {}]", self.x, self.y, self.z))
    }
}

gen_ops!(
    <S>;
    types Vec3<S> => Vec3<S>;
    for - call |v: &Vec3<S>| {
        vec3(-v.x, -v.y, -v.z)
    };
    where S: Scalar
);

gen_ops!(
    <S>;
    types Vec3<S>, Vec3<S> => Vec3<S>;
    for + call |l: &Vec3<S>, r: &Vec3<S>| {
        vec3(l.x + r.x, l.y + r.y, l.z + r.z)
    };
    for - call |l: &Vec3<S>, r: &Vec3<S>| {
        vec3(l.x - r.x, l.y - r.y, l.z - r.z)
    };
    for * call |l: &Vec3<S>, r: &Vec3<S>| {
        vec3(l.x * r.x, l.y * r.y, l.z * r.z)
    };
    for / call |l: &Vec3<S>, r: &Vec3<S>| {
        vec3(l.x / r.x, l.y / r.y, l.z / r.z)
    };
    where S: Scalar
);

gen_ops!(
    <S>;
    types Vec3<S>, Mat33<S> => Vec3<S>;
    for * call |l: &Vec3<S>, r: &Mat33<S>| {
        vec3(
            l.x * r[0][0] + l.y * r[1][0] + l.z * r[2][0],
            l.x * r[0][1] + l.y * r[1][1] + l.z * r[2][1],
            l.x * r[0][2] + l.y * r[1][2] + l.z * r[2][2],
        )
    };
    where S: Scalar
);

gen_ops!(
    <S>;
    types Mat33<S>, Vec3<S> => Vec3<S>;
    for * call |l: &Mat33<S>, r: &Vec3<S>| {
        vec3(
            l[0][0] * r.x + l[0][1] * r.y + l[0][2] * r.z,
            l[1][0] * r.x + l[1][1] * r.y + l[1][2] * r.z,
            l[2][0] * r.x + l[2][1] * r.y + l[2][2] * r.z,
        )
    };
    where S: Scalar
);

gen_ops!(
    <S>;
    types Vec3<S>, S => Vec3<S>;
    for + call |l: &Vec3<S>, r: &S| {
        vec3(l.x + *r, l.y + *r, l.z + *r)
    };
    for - call |l: &Vec3<S>, r: &S| {
        vec3(l.x - *r, l.y - *r, l.z - *r)
    };
    for * call |l: &Vec3<S>, r: &S| {
        vec3(l.x * *r, l.y * *r, l.z * *r)
    };
    for / call |l: &Vec3<S>, r: &S| {
        vec3(l.x / *r, l.y / *r, l.z / *r)
    };
    where S: Scalar
);

/*
// Unary
impl_op_ex!(-|a: Vec3| -> Vec3 { vec3(-a.x, -a.y, -a.z) });

// Binary non-commutative
impl_op_ex!(+|a: &Vec3, b: &Vec3| -> Vec3 { vec3(a.x + b.x, a.y + b.y, a.z + b.z) });
impl_op_ex!(-|a: &Vec3, b: &Vec3| -> Vec3 { vec3(a.x - b.x, a.y - b.y, a.z - b.z) });
impl_op_ex!(*|a: &Vec3, b: &Vec3| -> Vec3 { vec3(a.x * b.x, a.y * b.y, a.z * b.z) });
impl_op_ex!(/|a: &Vec3, b: &Vec3| -> Vec3 { vec3(a.x / b.x, a.y / b.y, a.z / b.z) });

// Assignment
impl_op_ex!(+= |a: &mut Vec3, b: &Vec3| {
   a.x += b.x;
   a.y += b.y;
   a.z += b.z;
});
impl_op_ex!(-= |a: &mut Vec3, b: &Vec3| {
   a.x -= b.x;
   a.y -= b.y;
   a.z -= b.z;
});

impl_op_ex_commutative!(*|v: &Vec3, s: &f64| -> Vec3 { vec3(v.x * s, v.y * s, v.z * s) });
impl_op_ex!(-|v: &Vec3, s: &f64| -> Vec3 { vec3(v.x - s, v.y - s, v.z - s) });
impl_op_ex!(/|v: &Vec3, s: &f64| -> Vec3 { vec3(v.x / s, v.y / s, v.z / s) });
impl_op_ex!(/|s: &f64, v: &Vec3| -> Vec3 { vec3(s / v.x, s / v.y, s / v.z) });
*/

#[cfg(test)]
mod tests {
    // TODO
}
