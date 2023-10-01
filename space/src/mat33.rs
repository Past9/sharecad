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

    pub fn rotation_from_axes(x: Vec2, y: Vec2) -> Self {
        // Arithmetic average of angle between local and global X-axes
        let x_axis_angle = (x.y.asin() + x.x.acos()) / 2.0;
        // Arithmetic average of angle between local and global Y-axes
        let y_axis_angle = (y.x.asin() + y.y.acos()) / 2.0;

        // Arithmetic average of X and Y angles
        let angle = (x_axis_angle + y_axis_angle) / 2.0;

        Self::rotation(rad(angle))
    }

    pub fn zero_translation(&self) -> Self {
        Self([
            [self[0][0], self[0][1], 0.0],
            [self[1][0], self[1][1], 0.0],
            [self[2][0], self[2][1], self[2][2]],
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

#[cfg(test)]
mod tests {
    use crate::{deg, point2};

    use super::*;

    #[test]
    fn rotates_point() {
        // Rotate +90°
        assert_cc!(
            point2(-2.0, 4.0),
            point2(4.0, 2.0).transform(Mat33::rotation(deg(90.0)))
        );

        // Rotate -90°
        assert_cc!(
            point2(2.0, -4.0),
            point2(4.0, 2.0).transform(Mat33::rotation(deg(-90.0)))
        );

        // Rotate all the way around, should get back to the starting point
        let rotation = Mat33::rotation(deg(36.0));
        assert_cc!(
            point2(4.0, 2.0),
            point2(4.0, 2.0)
                // Rotate 36° at a time, 10 times
                .transform(&rotation)
                .transform(&rotation)
                .transform(&rotation)
                .transform(&rotation)
                .transform(&rotation)
                .transform(&rotation)
                .transform(&rotation)
                .transform(&rotation)
                .transform(&rotation)
                .transform(&rotation)
        );

        // Same thing, going the other way
        let rotation = Mat33::rotation(deg(-36.0));
        assert_cc!(
            point2(4.0, 2.0),
            point2(4.0, 2.0)
                // Rotate -36° at a time, 10 times
                .transform(&rotation)
                .transform(&rotation)
                .transform(&rotation)
                .transform(&rotation)
                .transform(&rotation)
                .transform(&rotation)
                .transform(&rotation)
                .transform(&rotation)
                .transform(&rotation)
                .transform(&rotation)
        );
    }

    #[test]
    fn translates_point() {
        assert_cc!(
            point2(5.5, -0.3),
            point2(4.0, 2.0).transform(Mat33::translation(vec2(1.5, -2.3)))
        );
        assert_cc!(
            point2(4.0, 2.0),
            point2(5.5, -0.3).transform(Mat33::translation(vec2(-1.5, 2.3)))
        );
    }

    #[test]
    fn rotates_then_translates() {
        assert_cc!(
            point2(1.0, 5.0),
            point2(4.0, 2.0)
                .transform(Mat33::translation(vec2(3.0, 1.0)) * Mat33::rotation(deg(90.0)))
        );
    }

    #[test]
    fn translates_then_rotates() {
        assert_cc!(
            point2(-3.0, 7.0),
            point2(4.0, 2.0)
                .transform(Mat33::rotation(deg(90.0)) * Mat33::translation(vec2(3.0, 1.0)))
        );
    }
}
