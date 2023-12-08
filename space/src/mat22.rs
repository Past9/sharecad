use crate::{vec2, Angle, Point3, Quat, Vec2};
use auto_ops::{impl_op_ex, impl_op_ex_commutative};
use std::ops::Index;

#[derive(Copy, Clone)]
pub struct Mat22(pub [[f64; 2]; 2]);
impl Mat22 {
    pub const IDENTITY: Self = Self([[1.0, 0.0], [0.0, 1.0]]);

    pub fn new(a: f64, b: f64, c: f64, d: f64) -> Self {
        Self([[a, b], [c, d]])
    }

    pub fn transpose(&self) -> Self {
        Self([[self[0][0], self[1][0]], [self[0][1], self[1][1]]])
    }

    pub fn determinant(&self) -> f64 {
        let a = self[0][0];
        let b = self[0][1];
        let c = self[1][0];
        let d = self[1][1];

        (a * d) - (b * c)
    }

    pub fn adjoint(&self) -> Self {
        Self([[self[1][1], -self[0][1]], [-self[1][0], self[0][0]]])
    }

    pub fn inverse(&self) -> Option<Self> {
        let det = self.determinant();
        if det == 0.0 {
            None
        } else {
            Some(det.recip() * self.adjoint())
        }
    }

    pub fn approx_eq(&self, other: Self, tol: f64) -> bool {
        let mut equal = true;
        for r in 0..2 {
            for c in 0..2 {
                if (self[r][c] - other[r][c]).abs() > tol {
                    equal = false;
                }
            }
        }
        equal
    }
}
impl Index<usize> for Mat22 {
    type Output = [f64; 2];

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}
impl From<Mat22> for [[f64; 2]; 2] {
    fn from(mat: Mat22) -> Self {
        mat.0
    }
}
impl From<Mat22> for [[f32; 2]; 2] {
    fn from(mat: Mat22) -> Self {
        [
            [mat[0][0] as f32, mat[0][1] as f32],
            [mat[1][0] as f32, mat[1][1] as f32],
        ]
    }
}
impl std::fmt::Debug for Mat22 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:?}", self.0))
    }
}

impl_op_ex!(*|a: Mat22, b: Mat22| -> Self {
    Self([
        [
            a[0][0] * b[0][0] + a[0][1] * b[1][0],
            a[0][0] * b[0][1] + a[0][1] * b[1][1],
        ], //
        [
            a[1][0] * b[0][0] + a[1][1] * b[1][0],
            a[1][0] * b[0][1] + a[1][1] * b[1][1],
        ], //
    ])
});

impl_op_ex_commutative!(*|s: f64, m: Mat22| -> Mat22 {
    Mat22::new(m[0][0] * s, m[0][1] * s, m[1][0] * s, m[1][1] * s)
});

#[cfg(test)]
mod tests {
    use super::*;

    fn approx_eq(a: Mat22, b: Mat22) {
        if !a.approx_eq(b, 1e-9) {
            panic!("Matrices not approximaltey equal: {:?}, {:?}", a, b);
        }
    }

    #[test]
    fn inverts_matrix() {
        let m = Mat22::new(1.0, 2.0, 3.0, 4.0);
        let inv = Mat22::new(-2.0, 1.0, 1.5, -0.5);

        approx_eq(m.inverse().unwrap(), inv);
        approx_eq(inv.inverse().unwrap(), m);
        approx_eq(m * inv, Mat22::IDENTITY);
        approx_eq(inv * m, Mat22::IDENTITY);
        approx_eq(m * m.inverse().unwrap(), Mat22::IDENTITY);
        approx_eq(m.inverse().unwrap() * m, Mat22::IDENTITY);
    }
}
