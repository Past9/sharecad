use super::{Coincidence, Interval, Scalar};
use gen_ops::gen_ops;
use std::ops::Index;

#[derive(Copy, Clone)]
pub struct Mat22<S: Scalar>(pub [[S; 2]; 2]);
impl<S: Scalar> Mat22<S> {
    pub const IDENTITY: Self = Self([[S::ONE, S::ZERO], [S::ZERO, S::ONE]]);

    pub fn new(a: S, b: S, c: S, d: S) -> Self {
        Self([[a, b], [c, d]])
    }

    pub fn transpose(self) -> Self {
        Self([[self[0][0], self[1][0]], [self[0][1], self[1][1]]])
    }

    pub fn determinant(self) -> S {
        let a = self[0][0];
        let b = self[0][1];
        let c = self[1][0];
        let d = self[1][1];

        (a * d) - (b * c)
    }

    pub fn adjoint(self) -> Self {
        Self([[self[1][1], -self[0][1]], [-self[1][0], self[0][0]]])
    }

    /*
    pub fn approx_eq(self, other: Self, tol: S) -> bool {
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
     */
    pub fn el_mul(self, other: Self) -> Self {
        Self::new(
            self[0][0] * other[0][0],
            self[0][1] * other[0][1],
            self[1][0] * other[1][0],
            self[1][1] * other[1][1],
        )
    }
}
impl Mat22<f64> {
    pub fn as_interval(&self) -> Mat22<Interval> {
        Mat22::new(
            Interval::thin(self[0][0]),
            Interval::thin(self[0][1]),
            Interval::thin(self[1][0]),
            Interval::thin(self[1][1]),
        )
    }

    pub fn inverse(self) -> Option<Self> {
        let det = self.determinant();
        if det == 0.0 {
            None
        } else {
            Some(det.recip() * self.adjoint())
        }
    }

    pub fn signum(self) -> Self {
        Self::new(
            self[0][0].signum(),
            self[0][1].signum(),
            self[1][0].signum(),
            self[1][1].signum(),
        )
    }

    /*
    pub fn abs(self) -> Self {
        Self::new(
            self[0][0].abs(),
            self[0][1].abs(),
            self[1][0].abs(),
            self[1][1].abs(),
        )
    }
     */

    pub fn eigenvalues(&self) -> Vec<f64> {
        let a = self[0][0];
        let b = self[0][1];
        let c = self[1][0];
        let d = self[1][1];

        let root = (a.powi(2) + 4.0 * b * c - 2.0 * a * d + d.powi(2)).sqrt();

        let e0 = (a + d - root) / 2.0;
        let e1 = (a + d + root) / 2.0;

        vec![e0, e1]
    }

    pub fn spectral_radius(&self) -> f64 {
        let mut ev_abs = self
            .eigenvalues()
            .into_iter()
            .map(|ev| ev.abs())
            .collect::<Vec<_>>();

        ev_abs.sort_by(|a, b| a.total_cmp(b));

        ev_abs.into_iter().last().unwrap()
    }

    pub fn tys() -> Vec<Self> {
        vec![
            Self::new(1.0, 0.0, 0.0, 1.0),
            Self::new(1.0, 0.0, 0.0, -1.0),
            Self::new(-1.0, 0.0, 0.0, 1.0),
            Self::new(-1.0, 0.0, 0.0, -1.0),
        ]
    }
}
impl Mat22<Interval> {
    /*
    pub fn inverse(self) -> Option<Self> {
        let det = self.determinant();
        if det == Interval::ZERO {
            None
        } else {
            Some(det.recip() * self.adjoint())
        }
    }
    */

    pub fn signaccord(A: Mat22<f64>, B: Mat22<f64>, b: f64) -> Mat22<f64> {
        todo!()
    }

    /*
    pub fn inverse(self) -> Option<Self> {
        //let mut bys = vec![];
        for ty in Mat22::<f64>::tys() {
            let A = self.mid();
            let B = -ty * self.rad();
            let b = todo!();
            //let (x, S, flag) = Self::signaccord(A, B, b);
        }

        todo!()
    }
     */

    /*
    pub fn inverse(self) -> Option<Self> {
        let mid = self.mid();
        let rad = self.rad();

        let inv_mid = mid.inverse().unwrap();

        let mid_det = mid.determinant();
        let expand_rad = mid_det.recip().abs() * rad;

        Some(Self::new(
            Interval::from_mid_rad(inv_mid[0][0], expand_rad[0][0]),
            Interval::from_mid_rad(inv_mid[0][1], expand_rad[0][1]),
            Interval::from_mid_rad(inv_mid[1][0], expand_rad[1][0]),
            Interval::from_mid_rad(inv_mid[1][1], expand_rad[1][1]),
        ))
    }
     */

    /*
    pub fn inverse(self) -> Option<Self> {
        self.mid().inverse().map(|inv| inv.as_interval())
    }
      */

    pub fn is_regular(&self) -> bool {
        (self.mid().inverse().unwrap().determinant() * self.rad()).spectral_radius() < 1.0
    }

    pub fn split_on_zero(&self) -> Vec<Self> {
        let ivls_00 = self[0][0].split_on_zero();
        let ivls_01 = self[0][1].split_on_zero();
        let ivls_10 = self[1][0].split_on_zero();
        let ivls_11 = self[1][1].split_on_zero();

        let mut split_mats = vec![];
        for i00 in ivls_00.iter() {
            for i01 in ivls_01.iter() {
                for i10 in ivls_10.iter() {
                    for i11 in ivls_11.iter() {
                        let mat = Mat22::new(*i00, *i01, *i10, *i11);
                        if !mat.is_empty() {
                            split_mats.push(mat);
                        }
                    }
                }
            }
        }
        split_mats

        /*
        let x_ivls = self.x.split_on_zero();
        let y_ivls = self.y.split_on_zero();
        let mut split_vecs = vec![];
        for x in x_ivls.iter() {
            for y in y_ivls.iter() {
                split_vecs.push(Mat22::new(*x, *y));
            }
        }

        split_vecs
         */
    }

    pub fn mid(&self) -> Mat22<f64> {
        Mat22::new(
            self[0][0].mid(),
            self[0][1].mid(),
            self[1][0].mid(),
            self[1][1].mid(),
        )
    }

    pub fn rad(&self) -> Mat22<f64> {
        Mat22::new(
            self[0][0].rad(),
            self[0][1].rad(),
            self[1][0].rad(),
            self[1][1].rad(),
        )
    }

    pub fn inf(&self) -> Mat22<f64> {
        Mat22::new(
            self[0][0].inf(),
            self[0][1].inf(),
            self[1][0].inf(),
            self[1][1].inf(),
        )
    }

    pub fn sup(&self) -> Mat22<f64> {
        Mat22::new(
            self[0][0].sup(),
            self[0][1].sup(),
            self[1][0].sup(),
            self[1][1].sup(),
        )
    }

    pub fn from_bounds(inf: Mat22<f64>, sup: Mat22<f64>) -> Mat22<Interval> {
        Self::new(
            Interval::from_unordered(inf[0][0], sup[0][0]),
            Interval::from_unordered(inf[0][1], sup[0][1]),
            Interval::from_unordered(inf[1][0], sup[1][0]),
            Interval::from_unordered(inf[1][1], sup[1][1]),
        )
    }

    pub fn intersection(&self, other: Self) -> Self {
        let intersect_00 = self[0][0].intersection(other[0][0]);
        let intersect_01 = self[0][1].intersection(other[0][1]);
        let intersect_10 = self[1][0].intersection(other[1][0]);
        let intersect_11 = self[1][1].intersection(other[1][1]);

        if !intersect_00.is_empty()
            && !intersect_01.is_empty()
            && !intersect_10.is_empty()
            && !intersect_11.is_empty()
        {
            Mat22::new(intersect_00, intersect_01, intersect_10, intersect_11)
        } else {
            Mat22::new(
                Interval::EMPTY,
                Interval::EMPTY,
                Interval::EMPTY,
                Interval::EMPTY,
            )
        }
    }

    pub fn is_empty(&self) -> bool {
        self[0][0].is_empty()
            || self[0][1].is_empty()
            || self[1][0].is_empty()
            || self[1][1].is_empty()
    }
}
impl<S: Scalar> Index<usize> for Mat22<S> {
    type Output = [S; 2];

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}
impl<S: Scalar> From<Mat22<S>> for [[f64; 2]; 2] {
    fn from(mat: Mat22<S>) -> Self {
        [
            [mat[0][0].as_f64(), mat[0][1].as_f64()],
            [mat[1][0].as_f64(), mat[1][1].as_f64()],
        ]
    }
}
impl<S: Scalar> From<Mat22<S>> for [[f32; 2]; 2] {
    fn from(mat: Mat22<S>) -> Self {
        [
            [mat[0][0].as_f32(), mat[0][1].as_f32()],
            [mat[1][0].as_f32(), mat[1][1].as_f32()],
        ]
    }
}
impl<S: Scalar> std::fmt::Debug for Mat22<S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:?}", self.0))
    }
}

gen_ops!(
    <S>;
    types Mat22<S> => Mat22<S>;
    for - call |m: &Mat22<S>| {
        Self([
            [
                -m[0][0],
                -m[0][1],
            ],
            [
                -m[1][0],
                -m[1][1],
            ],
        ])
    };
    where S: Scalar
);

gen_ops!(
    <S>;
    types Mat22<S>, Mat22<S> => Mat22<S>;
    for - call |l: &Mat22<S>, r: &Mat22<S>| {
        Self([
            [
                l[0][0] - r[0][0],
                l[0][1] - r[0][1],
            ],
            [
                l[1][0] - r[1][0],
                l[1][1] - r[1][1],
            ],
        ])
    };
    for * call |l: &Mat22<S>, r: &Mat22<S>| {
        Self([
            [
                l[0][0] * r[0][0] + l[0][1] * r[1][0],
                l[0][0] * r[0][1] + l[0][1] * r[1][1],
            ],
            [
                l[1][0] * r[0][0] + l[1][1] * r[1][0],
                l[1][0] * r[0][1] + l[1][1] * r[1][1],
            ],
        ])
    };
    where S: Scalar
);

gen_ops!(
    <S>;
    types Mat22<S>, S => Mat22<S>;
    for * call |l: &Mat22<S>, r: &S| {
        Self([
            [
                l[0][0] * *r,
                l[0][1] * *r,
            ],
            [
                l[1][0] * *r,
                l[1][1] * *r,
            ],
        ])
    };
    for / call |l: &Mat22<S>, r: &S| {
        Self([
            [
                l[0][0] / *r,
                l[0][1] / *r,
            ],
            [
                l[1][0] / *r,
                l[1][1] / *r,
            ],
        ])
    };
    where S: Scalar
);

#[cfg(test)]
mod tests {
    use crate::math::Coincidence;

    use super::*;

    fn approx_eq<S: Scalar>(a: Mat22<S>, b: Mat22<S>) {
        if !a.cc_tol(b, 1e-9) {
            panic!("Matrices not approximately equal: {:?}, {:?}", a, b);
        }
    }

    #[test]
    fn inverts_matrix() {
        let m = Mat22::new(1.0, 2.0, 3.0, 4.0);
        let inv = Mat22::new(-2.0, 1.0, 1.5, -0.5);

        println!("f64 m = {:#?}", m);
        println!("f64 inv = {:#?}", inv);
        println!("f64 inv * m = {:#?}", inv * m);

        approx_eq(m.inverse().unwrap(), inv);
        approx_eq(inv.inverse().unwrap(), m);
        approx_eq(m * inv, Mat22::IDENTITY);
        approx_eq(inv * m, Mat22::IDENTITY);
        approx_eq(m * m.inverse().unwrap(), Mat22::IDENTITY);
        approx_eq(m.inverse().unwrap() * m, Mat22::IDENTITY);
    }

    /*
    #[test]
    fn inverts_interval_matrix() {
        let m = Mat22::new(1.0, 2.0, 3.0, 4.0).as_interval();
        let inv = m.inverse().unwrap();
        println!("ivl m = {:#?}", m);
        println!("ivl inv = {:#?}", inv);
        println!("ivl inv * m = {:#?}", inv * m);

        approx_eq(m.inverse().unwrap(), inv);
        approx_eq(inv.inverse().unwrap(), m);
        approx_eq(m * inv, Mat22::IDENTITY);
        approx_eq(inv * m, Mat22::IDENTITY);
        approx_eq(m * m.inverse().unwrap(), Mat22::IDENTITY);
        approx_eq(m.inverse().unwrap() * m, Mat22::IDENTITY);
    }
     */
}
