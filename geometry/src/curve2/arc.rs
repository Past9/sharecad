use std::f64::consts::PI;

use space::{point2, Mat33, Point2};

use crate::Curve2Impl;

pub fn arc(l: Mat33, a: f64, b: f64) -> Arc {
    Arc::new(l, a, b)
}

#[derive(Debug, Clone)]
pub struct Arc {
    pub l: Mat33,
    pub a: f64,
    pub b: f64,
}
impl Arc {
    pub fn new(l: Mat33, a: f64, b: f64) -> Self {
        Self { l, a, b }
    }
}
impl Curve2Impl for Arc {
    fn eval(&self, u: f64) -> Point2 {
        point2(self.a * u.cos(), self.b * u.sin()).transform(self.l)
    }

    fn u_min(&self) -> f64 {
        0.0
    }

    fn u_max(&self) -> f64 {
        2.0 * PI
    }

    fn der1(&self, u: f64) -> space::Vec2 {
        point2(-self.a * u.sin(), self.b * u.cos())
            .transform(self.l.zero_translation())
            .into_vec()
    }

    fn der2(&self, u: f64) -> space::Vec2 {
        point2(-self.a * u.cos(), -self.b * u.sin())
            .transform(self.l.zero_translation())
            .into_vec()
    }
}

#[cfg(test)]
mod tests {
    use std::{
        f64::consts::FRAC_PI_2,
        f64::consts::{PI, TAU},
    };

    use space::{assert_cc, deg, vec2, Coincidence};

    use crate::curve2::tests::{validate_der1, validate_der2};

    use super::*;

    #[test]
    fn tangents() {
        let arc = arc(Mat33::IDENTITY, 1.0, 1.0);

        let samples = 8;
        for i in 0..=samples {
            let u = i as f64 / samples as f64;

            println!("\n");
            println!("der1 {}", arc.der1(u));
            println!("tangent {}", arc.tangent(u));
            println!("der2 {}", arc.der2(u));
            println!("normal {}", arc.normal(u));
            println!("der1 orth {}", arc.der1(u).orthogonal());
            println!("tangent orth {}", arc.tangent(u).orthogonal());
        }
    }

    #[test]
    fn arc_test() {
        let arc = arc(
            Mat33::rotation(deg(90.0)) * Mat33::translation(vec2(3.0, 3.0)),
            2.0,
            1.0,
        );

        //println!("arc {:#?}", arc);

        let samples = 100;
        for i in 0..=samples {
            let u = i as f64 / samples as f64;

            println!("{}", arc.eval(u));
        }
    }

    #[test]
    fn curvature() {
        let arc = arc(Mat33::IDENTITY, 2.0, 1.0);

        assert_cc!(2.0, arc.curvature(0.0));
        assert_cc!(0.25, arc.curvature(TAU * 0.25));
        assert_cc!(2.0, arc.curvature(TAU * 0.5));
        assert_cc!(0.25, arc.curvature(TAU * 0.75));
        assert_cc!(2.0, arc.curvature(TAU));
    }

    #[test]
    fn arc_validate_der1() {
        validate_der1(
            &arc(
                Mat33::rotation(deg(90.0)) * Mat33::translation(vec2(3.0, 3.0)),
                2.0,
                1.0,
            ),
            100,
            0.0000001,
        );
    }

    #[test]
    fn arc_validate_der2() {
        validate_der2(
            &arc(
                Mat33::rotation(deg(90.0)) * Mat33::translation(vec2(3.0, 3.0)),
                2.0,
                1.0,
            ),
            100,
            0.0000001,
        );
    }
}
