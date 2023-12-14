use crate::Point3;
use auto_ops::{impl_op_ex, impl_op_ex_commutative};

pub fn vec3(x: f64, y: f64, z: f64) -> Vec3 {
    Vec3::new(x, y, z)
}

pub fn vec3_f32s(x: f32, y: f32, z: f32) -> Vec3 {
    Vec3::new(x as f64, y as f64, z as f64)
}

#[derive(Copy, Clone)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}
impl Vec3 {
    pub const ONES: Self = Self {
        x: 1.0,
        y: 1.0,
        z: 1.0,
    };
    pub const ZERO: Self = Self {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };
    pub const UNIT_X: Self = Self {
        x: 1.0,
        y: 0.0,
        z: 0.0,
    };
    pub const UNIT_Y: Self = Self {
        x: 0.0,
        y: 1.0,
        z: 0.0,
    };
    pub const UNIT_Z: Self = Self {
        x: 0.0,
        y: 0.0,
        z: 1.0,
    };

    pub fn is_zero(&self) -> bool {
        self.x == 0.0 && self.y == 0.0 && self.z == 0.0
    }

    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn magnitude2(&self) -> f64 {
        self.x.powi(2) + self.y.powi(2) + self.z.powi(2)
    }

    pub fn magnitude(&self) -> f64 {
        self.magnitude2().sqrt()
    }

    pub fn dot(&self, other: Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: Self) -> Self {
        Self {
            x: (self.y * other.z) - (self.z * other.y),
            y: (self.z * other.x) - (self.x * other.z),
            z: (self.x * other.y) - (self.y * other.x),
        }
    }

    pub fn normalize(&self) -> Self {
        let mag = self.magnitude();
        Self {
            x: self.x / mag,
            y: self.y / mag,
            z: self.z / mag,
        }
    }

    pub fn into_point(&self) -> Point3 {
        (*self).into()
    }

    pub fn to_f64s(&self) -> [f64; 3] {
        [self.x, self.y, self.z]
    }

    pub fn to_f32s(&self) -> [f32; 3] {
        [self.x as f32, self.y as f32, self.z as f32]
    }

    pub fn has_nan(&self) -> bool {
        self.x.is_nan() || self.y.is_nan() || self.z.is_nan()
    }

    pub fn dot_self(&self) -> f64 {
        self.dot(*self)
    }

    pub fn powi(&self, n: i32) -> Self {
        Self {
            x: self.x.powi(n),
            y: self.y.powi(n),
            z: self.z.powi(n),
        }
    }

    pub fn sum(&self) -> f64 {
        self.x + self.y + self.z
    }

    pub fn lerp(&self, other: Self, t: f64) -> Self {
        (Self::ONES - t) * self + t * other
    }
}
impl From<Point3> for Vec3 {
    fn from(point: Point3) -> Self {
        Self {
            x: point.x,
            y: point.y,
            z: point.z,
        }
    }
}
impl From<[f64; 3]> for Vec3 {
    fn from(floats: [f64; 3]) -> Self {
        Self {
            x: floats[0],
            y: floats[1],
            z: floats[2],
        }
    }
}
impl From<[f32; 3]> for Vec3 {
    fn from(floats: [f32; 3]) -> Self {
        Self {
            x: floats[0] as f64,
            y: floats[1] as f64,
            z: floats[2] as f64,
        }
    }
}
impl std::fmt::Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("[{}, {}, {}]", self.x, self.y, self.z))
    }
}
impl std::fmt::Debug for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("[{}, {}, {}]", self.x, self.y, self.z))
    }
}

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

#[cfg(test)]
mod tests {
    // TODO
}
