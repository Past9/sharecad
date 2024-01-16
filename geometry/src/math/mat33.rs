use super::{vec3, Quat, Scalar, Vec3};
use gen_ops::gen_ops;
use std::ops::Index;

#[derive(Copy, Clone)]
pub struct Mat33<S: Scalar>(pub [[S; 3]; 3]);
impl<S: Scalar> Mat33<S> {
    pub const IDENTITY: Self = Self([
        [S::ONE, S::ZERO, S::ZERO], //
        [S::ZERO, S::ONE, S::ZERO], //
        [S::ZERO, S::ZERO, S::ONE], //
    ]);

    pub fn new(a: S, b: S, c: S, d: S, e: S, f: S, g: S, h: S, i: S) -> Self {
        Self([[a, b, c], [d, e, f], [g, h, i]])
    }

    pub fn from_col_vecs(x: Vec3<S>, y: Vec3<S>, z: Vec3<S>) -> Self {
        Self([[x.x, y.x, z.x], [x.y, y.y, z.y], [x.z, y.z, z.z]])
    }

    pub fn into_axes(self) -> (Vec3<S>, Vec3<S>, Vec3<S>) {
        (
            vec3(self[0][0], self[1][0], self[2][0]),
            vec3(self[0][1], self[1][1], self[2][1]),
            vec3(self[0][2], self[1][2], self[2][2]),
        )
    }

    pub fn transpose(self) -> Self {
        Self([
            [self[0][0], self[1][0], self[2][0]],
            [self[0][1], self[1][1], self[2][1]],
            [self[0][2], self[1][2], self[2][2]],
        ])
    }

    pub fn zero_translation(self) -> Self {
        Self([
            [self[0][0], self[0][1], S::ZERO],
            [self[1][0], self[1][1], S::ZERO],
            [self[2][0], self[2][1], self[2][2]],
        ])
    }

    pub fn determinant(self) -> S {
        let a = self[0][0];
        let b = self[0][1];
        let c = self[0][2];
        let d = self[1][0];
        let e = self[1][1];
        let f = self[1][2];
        let g = self[2][0];
        let h = self[2][1];
        let i = self[2][2];

        (a * e * i) + (b * f * g) + (c * d * h) - (c * e * g) - (b * d * i) - (a * f * h)
    }

    pub fn inverse(self) -> Option<Self> {
        let det = self.determinant();
        if det == 0.0 {
            None
        } else {
            let a = self[0][0];
            let b = self[0][1];
            let c = self[0][2];
            let d = self[1][0];
            let e = self[1][1];
            let f = self[1][2];
            let g = self[2][0];
            let h = self[2][1];
            let i = self[2][2];

            let i_a = e * i - f * h;
            let i_b = -(d * i - f * g);
            let i_c = d * h - e * g;
            let i_d = -(b * i - c * h);
            let i_e = a * i - c * g;
            let i_f = -(a * h - b * g);
            let i_g = b * f - c * e;
            let i_h = -(a * f - c * d);
            let i_i = a * e - b * d;

            let inv =
                self.determinant().recip() * Self::new(i_a, i_d, i_g, i_b, i_e, i_h, i_c, i_f, i_i);

            Some(inv)
        }
    }

    pub fn approx_eq(self, other: Self, tol: f64) -> bool {
        let mut equal = true;
        for r in 0..3 {
            for c in 0..3 {
                if (self[r][c] - other[r][c]).abs() > tol {
                    equal = false;
                }
            }
        }
        equal
    }

    pub fn col0(self) -> Vec3<S> {
        vec3(self[0][0], self[1][0], self[2][0])
    }

    pub fn col1(self) -> Vec3<S> {
        vec3(self[0][1], self[1][1], self[2][1])
    }

    pub fn col2(self) -> Vec3<S> {
        vec3(self[0][2], self[1][2], self[2][2])
    }
}
impl<S: Scalar> Index<usize> for Mat33<S> {
    type Output = [S; 3];

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}
impl<S: Scalar> From<Mat33<S>> for [[f64; 3]; 3] {
    fn from(mat: Mat33<S>) -> Self {
        mat.0
    }
}
impl<S: Scalar> From<Mat33<S>> for [[f32; 3]; 3] {
    fn from(mat: Mat33<S>) -> Self {
        [
            [mat[0][0] as f32, mat[0][1] as f32, mat[0][2] as f32],
            [mat[1][0] as f32, mat[1][1] as f32, mat[1][2] as f32],
            [mat[2][0] as f32, mat[2][1] as f32, mat[2][2] as f32],
        ]
    }
}
impl<S: Scalar> From<Quat<S>> for Mat33<S> {
    fn from(quat: Quat<S>) -> Self {
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
        Mat33::new(
            S::ONE - yy2 - zz2,  xy2 - sz2,        xz2 + sy2,
            xy2 + sz2,        S::ONE - xx2 - zz2,  yz2 - sx2,
            xz2 - sy2,        yz2 + sx2,        S::ONE - xx2 - yy2,
        )
    }
}
impl<S: Scalar> std::fmt::Debug for Mat33<S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:?}", self.0))
    }
}

gen_ops!(
    <S>;
    types Mat33<S>, Mat33<S> => Mat33<S>;
    for * call |l: &Mat33<S>, r: &Mat33<S>| {
        Self([
            [
                l[0][0] * r[0][0] + l[0][1] * r[1][0] + l[0][2] * r[2][0],
                l[0][0] * r[0][1] + l[0][1] * r[1][1] + l[0][2] * r[2][1],
                l[0][0] * r[0][2] + l[0][1] * r[1][2] + l[0][2] * r[2][2],
            ],
            [
                l[1][0] * r[0][0] + l[1][1] * r[1][0] + l[1][2] * r[2][0],
                l[1][0] * r[0][1] + l[1][1] * r[1][1] + l[1][2] * r[2][1],
                l[1][0] * r[0][2] + l[1][1] * r[1][2] + l[1][2] * r[2][2],
            ],
            [
                l[2][0] * r[0][0] + l[2][1] * r[1][0] + l[2][2] * r[2][0],
                l[2][0] * r[0][1] + l[2][1] * r[1][1] + l[2][2] * r[2][1],
                l[2][0] * r[0][2] + l[2][1] * r[1][2] + l[2][2] * r[2][2],
            ],
        ])
    };
);

gen_ops!(
    <S>;
    types Mat33<S>, S => Mat33<S>;
    for * call |l: &Mat33<S>, r: &S| {
        Self([
            [
                l[0][0] * r,
                l[0][1] * r,
                l[0][2] * r,
            ],
            [
                l[1][0] * r,
                l[1][1] * r,
                l[1][2] * r,
            ],
            [
                l[2][0] * r,
                l[2][1] * r,
                l[2][2] * r,
            ],
        ])
    };
);

/*
impl_op_ex!(*|m: &Mat33, v: &Vec3| -> Vec3 {
    Vec3::new(
        m[0][0] * v.x + m[0][1] * v.y + m[0][2] * v.z,
        m[1][0] * v.x + m[1][1] * v.y + m[1][2] * v.z,
        m[2][0] * v.x + m[2][1] * v.y + m[2][2] * v.z,
    )
});

impl_op_ex!(*|v: &Vec3, m: &Mat33| -> Vec3 {
    Vec3::new(
        m[0][0] * v.x + m[1][0] * v.y + m[2][0] * v.z,
        m[0][1] * v.x + m[1][1] * v.y + m[2][1] * v.z,
        m[0][2] * v.x + m[1][2] * v.y + m[2][2] * v.z,
    )
});

impl_op_ex!(*|a: &Mat33, b: &Mat33| -> Mat33 {
    Mat33([
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

impl_op_ex!(-|a: &Mat33, b: &Mat33| -> Mat33 {
    Mat33([
        [a[0][0] - b[0][0], a[0][1] - b[0][1], a[0][2] - b[0][2]],
        [a[1][0] - b[1][0], a[1][1] - b[1][1], a[1][2] - b[1][2]],
        [a[2][0] - b[2][0], a[2][1] - b[2][1], a[2][2] - b[2][2]],
    ])
});

impl_op_ex!(+|a: &Mat33, b: &Mat33| -> Mat33 {
    Mat33([
        [a[0][0] + b[0][0], a[0][1] + b[0][1], a[0][2] + b[0][2]],
        [a[1][0] + b[1][0], a[1][1] + b[1][1], a[1][2] + b[1][2]],
        [a[2][0] + b[2][0], a[2][1] + b[2][1], a[2][2] + b[2][2]],
    ])
});

impl_op_ex_commutative!(*|s: f64, m: &Mat33| -> Mat33 {
    Mat33::new(
        m[0][0] * s,
        m[0][1] * s,
        m[0][2] * s,
        m[1][0] * s,
        m[1][1] * s,
        m[1][2] * s,
        m[2][0] * s,
        m[2][1] * s,
        m[2][2] * s,
    )
});
*/

/*
#[cfg(test)]
mod tests {
    use crate::math::{deg, point2};

    use super::*;

    fn approx_eq(a: Mat33, b: Mat33) {
        if !a.approx_eq(b, 1e-9) {
            panic!("Matrices not approximaltey equal: {:?}, {:?}", a, b);
        }
    }

    #[test]
    fn inverts_matrix() {
        let m = Mat33::new(1.0, 2.0, -1.0, 2.0, 1.0, 2.0, -1.0, 2.0, 1.0);

        let inv = Mat33::new(
            3.0 / 16.0,
            1.0 / 4.0,
            -5.0 / 16.0,
            1.0 / 4.0,
            0.0,
            1.0 / 4.0,
            -5.0 / 16.0,
            1.0 / 4.0,
            3.0 / 16.0,
        );

        approx_eq(m.inverse().unwrap(), inv);
        approx_eq(inv.inverse().unwrap(), m);
        approx_eq(m * inv, Mat33::IDENTITY);
        approx_eq(inv * m, Mat33::IDENTITY);
        approx_eq(m * m.inverse().unwrap(), Mat33::IDENTITY);
        approx_eq(m.inverse().unwrap() * m, Mat33::IDENTITY);
    }

    #[test]
    fn makes_rotation_matrix() {
        // +90° (counterclockwise 1/4 turn)
        assert_cc!(
            Mat33([
                [0.0, -1.0, 0.0], //
                [1.0, 0.0, 0.0],  //
                [0.0, 0.0, 1.0],  //
            ]),
            Mat33::rotation(deg(90.0))
        );
    }

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
                .transform(rotation)
                .transform(rotation)
                .transform(rotation)
                .transform(rotation)
                .transform(rotation)
                .transform(rotation)
                .transform(rotation)
                .transform(rotation)
                .transform(rotation)
                .transform(rotation)
        );

        // Same thing, going the other way
        let rotation = Mat33::rotation(deg(-36.0));
        assert_cc!(
            point2(4.0, 2.0),
            point2(4.0, 2.0)
                // Rotate -36° at a time, 10 times
                .transform(rotation)
                .transform(rotation)
                .transform(rotation)
                .transform(rotation)
                .transform(rotation)
                .transform(rotation)
                .transform(rotation)
                .transform(rotation)
                .transform(rotation)
                .transform(rotation)
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
*/
