use auto_ops::{impl_op_ex, impl_op_ex_commutative};

use super::{vec3, Mat44, Vec3};

pub fn point3(x: f64, y: f64, z: f64) -> Point3 {
    Point3::new(x, y, z)
}

pub fn point3_f32s(x: f32, y: f32, z: f32) -> Point3 {
    Point3::new(x as f64, y as f64, z as f64)
}

#[derive(Copy, Clone, PartialEq)]
pub struct Point3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}
impl Point3 {
    pub const ZERO: Self = Point3 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };
    pub const ORIGIN: Self = Point3::ZERO;

    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn into_vec(&self) -> Vec3 {
        (*self).into()
    }

    pub fn transform(&self, m: Mat44) -> Self {
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

    pub fn to_f64s(&self) -> [f64; 3] {
        [self.x, self.y, self.z]
    }

    pub fn to_f32s(&self) -> [f32; 3] {
        [self.x as f32, self.y as f32, self.z as f32]
    }
}
impl From<Vec3> for Point3 {
    fn from(vec: Vec3) -> Self {
        Self {
            x: vec.x,
            y: vec.y,
            z: vec.z,
        }
    }
}
impl std::fmt::Display for Point3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("({}, {}, {})", self.x, self.y, self.z))
    }
}
impl std::fmt::Debug for Point3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("({}, {}, {})", self.x, self.y, self.z))
    }
}

impl_op_ex_commutative!(+|p: &Vec3, v: &Point3| -> Point3 {
    point3(p.x + v.x, p.y + v.y, p.z + v.z)
});
impl_op_ex!(+|a: &Point3, b: &Point3| -> Point3 { point3(a.x + b.x, a.y + b.y, a.z + b.z) });
impl_op_ex!(-|p: &Point3, v: &Vec3| -> Point3 { point3(p.x - v.x, p.y - v.y, p.z - v.z) });
impl_op_ex!(-|a: &Point3, b: &Point3| -> Vec3 { vec3(a.x - b.x, a.y - b.y, a.z - b.z) });

// Binary commutative
impl_op_ex_commutative!(*|v: &Point3, s: f64| -> Point3 { point3(v.x * s, v.y * s, v.z * s) });
impl_op_ex_commutative!(/|v: &Point3, s: f64| -> Point3 { point3(v.x / s, v.y / s, v.z / s) });

// Assignment
impl_op_ex!(+= |a: &mut Point3, b: &Point3| {
   a.x += b.x;
   a.y += b.y;
   a.z += b.z;
});
impl_op_ex!(-= |a: &mut Point3, b: &Point3| {
   a.x -= b.x;
   a.y -= b.y;
   a.z -= b.z;
});
