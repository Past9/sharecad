use auto_ops::{impl_op_ex, impl_op_ex_commutative};

pub fn vec4(x: f64, y: f64, z: f64, w: f64) -> Vec4 {
    Vec4::new(x, y, z, w)
}

#[derive(Copy, Clone)]
pub struct Vec4 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}
impl Vec4 {
    pub const ONES: Self = Self {
        x: 1.0,
        y: 1.0,
        z: 1.0,
        w: 1.0,
    };

    pub const ZERO: Self = Self {
        x: 0.0,
        y: 0.0,
        z: 0.0,
        w: 0.0,
    };

    pub const UNIT_X: Self = Self {
        x: 1.0,
        y: 0.0,
        z: 0.0,
        w: 0.0,
    };

    pub const UNIT_Y: Self = Self {
        x: 0.0,
        y: 1.0,
        z: 0.0,
        w: 0.0,
    };

    pub const UNIT_Z: Self = Self {
        x: 0.0,
        y: 0.0,
        z: 1.0,
        w: 0.0,
    };

    pub const UNIT_W: Self = Self {
        x: 0.0,
        y: 0.0,
        z: 0.0,
        w: 1.0,
    };

    pub fn new(x: f64, y: f64, z: f64, w: f64) -> Self {
        Self { x, y, z, w }
    }

    pub fn magnitude2(&self) -> f64 {
        self.dot(*self)
    }

    pub fn magnitude(&self) -> f64 {
        self.magnitude2().sqrt()
    }

    pub fn dot(&self, other: Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w
    }

    pub fn normalize(&self) -> Self {
        let mag = self.magnitude();
        self / mag
    }
}
impl std::fmt::Display for Vec4 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "[{}, {}, {}, {}]",
            self.x, self.y, self.z, self.w
        ))
    }
}
impl std::fmt::Debug for Vec4 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "[{}, {}, {}, {}]",
            self.x, self.y, self.z, self.w
        ))
    }
}

impl_op_ex!(-|a: &Vec4| -> Vec4 { vec4(-a.x, -a.y, -a.z, -a.w,) });

impl_op_ex_commutative!(*|v: &Vec4, s: f64| -> Vec4 { vec4(v.x * s, v.y * s, v.z * s, v.w * s,) });
impl_op_ex!(/|v: &Vec4, s: f64| -> Vec4 { vec4(v.x / s, v.y / s, v.z / s, v.w / s,) });
impl_op_ex!(/|s: f64, v: &Vec4| -> Vec4 { vec4(s / v.x, s / v.y, s / v.z, s / v.w) });

impl_op_ex!(-|a: &Vec4, b: &Vec4| -> Vec4 { vec4(a.x - b.x, a.y - b.y, a.z - b.z, a.w - b.w,) });
impl_op_ex!(+|a: &Vec4, b: &Vec4| -> Vec4 { vec4(a.x + b.x, a.y + b.y, a.z + b.z, a.w + b.w,) });
impl_op_ex!(*|a: &Vec4, b: &Vec4| -> Vec4 { vec4(a.x * b.x, a.y * b.y, a.z * b.z, a.w * b.w,) });
impl_op_ex!(/|a: &Vec4, b: &Vec4| -> Vec4 { vec4(a.x / b.x, a.y / b.y, a.z / b.z, a.w / b.w,) });
