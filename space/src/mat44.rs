use std::ops::Index;

use auto_ops::impl_op_ex;

use crate::{Point3, Quat, Vec3, Mat33};

#[derive(Copy, Clone)]
pub struct Mat44(pub [[f64; 4]; 4]);
impl Mat44 {
    pub const IDENTITY: Self = Self([
        [1.0, 0.0, 0.0, 0.0], //
        [0.0, 1.0, 0.0, 0.0], //
        [0.0, 0.0, 1.0, 0.0], //
        [0.0, 0.0, 0.0, 1.0], //
    ]);

    pub fn new(
        a: f64,
        b: f64,
        c: f64,
        d: f64,
        e: f64,
        f: f64,
        g: f64,
        h: f64,
        i: f64,
        j: f64,
        k: f64,
        l: f64,
        m: f64,
        n: f64,
        o: f64,
        p: f64,
    ) -> Self {
        Self([[a, b, c, d], [e, f, g, h], [i, j, k, l], [m, n, o, p]])
    }

    pub fn transpose(&self) -> Self {
        let m = self.0;
        Self([
            [m[0][0], m[1][0], m[2][0], m[3][0]],
            [m[0][1], m[1][1], m[2][1], m[3][1]],
            [m[0][2], m[1][2], m[2][2], m[3][2]],
            [m[0][3], m[1][3], m[2][3], m[3][3]],
        ])
    }

    pub fn translation(vec: Vec3) -> Self {
        Self([
            [1.0, 0.0, 0.0, vec.x], //
            [0.0, 1.0, 0.0, vec.y], //
            [0.0, 0.0, 1.0, vec.z], //
            [0.0, 0.0, 0.0, 1.0],   //
        ])
    }

    pub fn scale(vec: Vec3) -> Self {
        Self([
            [vec.x, 0.0, 0.0, 0.0], //
            [0.0, vec.y, 0.0, 0.0], //
            [0.0, 0.0, vec.z, 0.0], //
            [0.0, 0.0, 0.0, 1.0],   //
        ])
    }

    pub fn look_to_rh_rotation(dir: Vec3, up: Vec3) -> Self {
        let z = dir.normalize();
        let y = up.normalize();
        let x = y.cross(z).normalize();

        #[cfg_attr(rustfmt, rustfmt_skip)]
        let rotation = Mat44::new(
            x.x, x.y, x.z, 0.0,
            y.x, y.y, y.z, 0.0,
            z.x, z.y, z.z, 0.0,
            0.0, 0.0, 0.0, 1.0
        );

        rotation
    }

    pub fn look_to_rh_translation(eye: Point3) -> Self {
        let eye = eye.into_vec();

        #[cfg_attr(rustfmt, rustfmt_skip)]
        let translation = Mat44::new(
            1.0, 0.0, 0.0, -eye.x,
            0.0, 1.0, 0.0, -eye.y,
            0.0, 0.0, 1.0, -eye.z,
            0.0, 0.0, 0.0, 1.0,
        );
    
        translation
    }

    pub fn look_to_rh(eye: Point3, dir: Vec3, up: Vec3) -> Self {
        Self::look_to_rh_rotation(dir, up) * Self::look_to_rh_translation(eye)
    }

    pub fn look_at_rh(eye: Point3, center: Point3, up: Vec3) -> Self {
        Self::look_to_rh(eye, center - eye, up)
    }

    pub fn look_at_rh_rotation(eye: Point3, center: Point3, up: Vec3) -> Self {
        Self::look_to_rh_rotation(center - eye, up)
    }

    pub fn determinant(&self) -> f64 {
        let a = self[0][0];
        let b = self[0][1];
        let c = self[0][2];
        let d = self[0][3];
        let e = self[1][0];
        let f = self[1][1];
        let g = self[1][2];
        let h = self[1][3];
        let i = self[2][0];
        let j = self[2][1];
        let k = self[2][2];
        let l = self[2][3];
        let m = self[3][0];
        let n = self[3][1];
        let o = self[3][2];
        let p = self[3][3];

        a * Mat33::new(f, g, h, j, k, l, n, o, p).determinant()
        - b * Mat33::new(e, g, h, i, k, l, m, o, p).determinant()
        + c * Mat33::new(e, f, h, i, j, l, m, n, p).determinant()
        - d * Mat33::new(e, g, h, i, j, k, m, n, o).determinant()
    }

    pub fn powi(&self, power: u32) -> Self {
        let mut result = Self::IDENTITY;
        for i in 0..power {
            result = result * *self;
        }
        result
    }

    pub fn approx_eq(&self, other: Self, tol: f64) -> bool {
        let mut equal = true;
        for r in 0..4 {
            for c in 0..4 {
                if (self[r][c] - other[r][c]).abs() > tol {
                    equal = false;
                }
            }
        } 
        equal
    }
}
impl Index<usize> for Mat44 {
    type Output = [f64; 4];

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}
impl From<Mat44> for [[f64; 4]; 4] {
    fn from(mat: Mat44) -> Self {
        mat.0
    }
}
impl From<Mat44> for [[f32; 4]; 4] {
    fn from(mat: Mat44) -> Self {
        let m = mat.0;
        [
            [
                m[0][0] as f32,
                m[0][1] as f32,
                m[0][2] as f32,
                m[0][3] as f32,
            ],
            [
                m[1][0] as f32,
                m[1][1] as f32,
                m[1][2] as f32,
                m[1][3] as f32,
            ],
            [
                m[2][0] as f32,
                m[2][1] as f32,
                m[2][2] as f32,
                m[2][3] as f32,
            ],
            [
                m[3][0] as f32,
                m[3][1] as f32,
                m[3][2] as f32,
                m[3][3] as f32,
            ],
        ]
    }
}
impl From<Quat> for Mat44 {
    fn from(quat: Quat) -> Self {
        let x2 = quat.v.x + quat.v.x;
        let y2 = quat.v.y + quat.v.y;
        let z2 = quat.v.z + quat.v.z;

        let xx2 = x2 * quat.v.x;
        let xy2 = x2 * quat.v.y;
        let xz2 = x2 * quat.v.z;

        let yy2 = y2 * quat.v.y;
        let yz2 = y2 * quat.v.z;
        let zz2 = z2 * quat.v.z;

        let sy2 = y2 * quat.s;
        let sz2 = z2 * quat.s;
        let sx2 = x2 * quat.s;

        #[cfg_attr(rustfmt, rustfmt_skip)]
        Mat44::new(
            1.0 - yy2 - zz2,  xy2 - sz2,        xz2 + sy2,        0.0,
            xy2 + sz2,        1.0 - xx2 - zz2,  yz2 - sx2,        0.0,
            xz2 - sy2,        yz2 + sx2,        1.0 - xx2 - yy2,  0.0,
            0.0,              0.0,              0.0,              1.0
        )
    }
}
impl std::fmt::Debug for Mat44 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:?}", self.0))
    }
}

impl_op_ex!(*|a: Mat44, b: Mat44| -> Self {
    #[cfg_attr(rustfmt, rustfmt_skip)]
    Self([
        [
            a[0][0] * b[0][0] + a[0][1] * b[1][0] + a[0][2] * b[2][0] + a[0][3] * b[3][0],
            a[0][0] * b[0][1] + a[0][1] * b[1][1] + a[0][2] * b[2][1] + a[0][3] * b[3][1],
            a[0][0] * b[0][2] + a[0][1] * b[1][2] + a[0][2] * b[2][2] + a[0][3] * b[3][2],
            a[0][0] * b[0][3] + a[0][1] * b[1][3] + a[0][2] * b[2][3] + a[0][3] * b[3][3],
        ], 
        [
            a[1][0] * b[0][0] + a[1][1] * b[1][0] + a[1][2] * b[2][0] + a[1][3] * b[3][0],
            a[1][0] * b[0][1] + a[1][1] * b[1][1] + a[1][2] * b[2][1] + a[1][3] * b[3][1],
            a[1][0] * b[0][2] + a[1][1] * b[1][2] + a[1][2] * b[2][2] + a[1][3] * b[3][2],
            a[1][0] * b[0][3] + a[1][1] * b[1][3] + a[1][2] * b[2][3] + a[1][3] * b[3][3],
        ],
        [
            a[2][0] * b[0][0] + a[2][1] * b[1][0] + a[2][2] * b[2][0] + a[2][3] * b[3][0],
            a[2][0] * b[0][1] + a[2][1] * b[1][1] + a[2][2] * b[2][1] + a[2][3] * b[3][1],
            a[2][0] * b[0][2] + a[2][1] * b[1][2] + a[2][2] * b[2][2] + a[2][3] * b[3][2],
            a[2][0] * b[0][3] + a[2][1] * b[1][3] + a[2][2] * b[2][3] + a[2][3] * b[3][3],
        ], 
        [
            a[3][0] * b[0][0] + a[3][1] * b[1][0] + a[3][2] * b[2][0] + a[3][3] * b[3][0],
            a[3][0] * b[0][1] + a[3][1] * b[1][1] + a[3][2] * b[2][1] + a[3][3] * b[3][1],
            a[3][0] * b[0][2] + a[3][1] * b[1][2] + a[3][2] * b[2][2] + a[3][3] * b[3][2],
            a[3][0] * b[0][3] + a[3][1] * b[1][3] + a[3][2] * b[2][3] + a[3][3] * b[3][3],
        ], 
    ])
});

#[cfg(test)]
mod tests {
    //
}
