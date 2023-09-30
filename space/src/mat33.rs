use std::ops::Index;

use auto_ops::impl_op_ex;

use crate::{rad, vec2, Angle, Vec2};

#[derive(Clone)]
pub struct Mat33(pub [[f64; 3]; 3]);
impl Mat33 {
    pub const IDENTITY: Self = Self([
        [1.0, 0.0, 0.0], //
        [0.0, 1.0, 0.0], //
        [0.0, 0.0, 1.0], //
    ]);

    pub fn new(a: f64, b: f64, c: f64, d: f64, e: f64, f: f64, g: f64, h: f64, i: f64) -> Self {
        Self([[a, b, c], [d, e, f], [g, h, i]])
    }

    pub fn translation(vec: Vec2) -> Self {
        Self([
            [1.0, 0.0, vec.x], //
            [0.0, 1.0, vec.y], //
            [0.0, 0.0, 1.0],   //
        ])
    }

    pub fn rotation(angle: Angle) -> Self {
        let cos = angle.cos();
        let sin = angle.sin();
        Self([
            [cos, -sin, 0.0], //
            [sin, cos, 0.0],  //
            [0.0, 0.0, 1.0],  //
        ])
    }

    pub fn get_translation(&self) -> Vec2 {
        vec2(self[0][2], self[1][2])
    }

    pub fn get_rotation(&self) -> Angle {
        // Arithmetic average of reversing the trig operations
        // used to create the rotation elements
        rad(
            (self[0][0].acos() + (-self[0][1]).asin() + self[1][0].asin() + self[1][1].acos())
                / 4.0,
        )
    }
}
impl Index<usize> for Mat33 {
    type Output = [f64; 3];

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}
impl std::fmt::Debug for Mat33 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:?}", self.0))
    }
}
/*
impl From<Coord2> for Mat33 {
    fn from(coord: Coord2) -> Self {
        coord.to_mat33()
    }
}
 */

impl_op_ex!(*|a: Mat33, b: Mat33| -> Self {
    Self([
        [
            a[0][0] * b[0][0] + a[0][1] * b[1][0] + a[0][2] * b[2][0],
            a[0][0] * b[0][1] + a[0][1] * b[1][1] + a[0][2] * b[2][1],
            a[0][0] * b[0][2] + a[0][1] * b[1][2] + a[0][2] * b[2][2],
        ], //
        [
            a[1][0] * b[0][0] + a[1][1] * b[1][0] + a[1][2] * b[2][0],
            a[1][0] * b[0][1] + a[1][1] * b[1][1] + a[1][2] * b[2][1],
            a[1][0] * b[0][2] + a[1][1] * b[1][2] + a[1][2] * b[2][2],
        ], //
        [
            a[2][0] * b[0][0] + a[2][1] * b[1][0] + a[2][2] * b[2][0],
            a[2][0] * b[0][1] + a[2][1] * b[1][1] + a[2][2] * b[2][1],
            a[2][0] * b[0][2] + a[2][1] * b[1][2] + a[2][2] * b[2][2],
        ], //
    ])
});
