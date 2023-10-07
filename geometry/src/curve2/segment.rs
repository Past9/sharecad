use space::{Point2, Vec2};

use crate::Curve2Impl;

pub fn segment(a: Point2, b: Point2) -> Segment {
    Segment::new(a, b)
}

#[derive(Debug, Clone)]
pub struct Segment {
    a: Point2,
    b: Point2,
}
impl Segment {
    pub fn new(a: Point2, b: Point2) -> Self {
        Self { a, b }
    }
}
impl Curve2Impl for Segment {
    fn u_min(&self) -> f64 {
        0.0
    }

    fn u_max(&self) -> f64 {
        1.0
    }

    fn eval(&self, u: f64) -> Point2 {
        ((1.0 - u) * self.a.into_vec() + u * self.b.into_vec()).into_point()
    }

    fn der1(&self, _u: f64) -> space::Vec2 {
        self.b - self.a
    }

    fn der2(&self, _u: f64) -> space::Vec2 {
        Vec2::ZERO
    }
}

#[cfg(test)]
mod tests {
    use space::{assert_cc, point2, vec2};

    use super::*;

    #[test]
    fn segment_test() {
        let segment = segment(point2(1.0, 2.0), point2(4.0, -3.0));

        let samples = 100;
        for i in 0..=samples {
            let u = i as f64 / samples as f64;

            println!("{}", segment.eval_normalized(u));
        }
    }

    #[test]
    fn der1() {
        let segment = segment(point2(2.0, 2.0), point2(7.0, 4.0));
        let der = vec2(5.0, 2.0);
        assert_cc!(der, segment.der1(0.0));
        assert_cc!(der, segment.der1(0.5));
        assert_cc!(der, segment.der1(1.0));
    }

    #[test]
    fn der2() {
        let segment = segment(point2(2.0, 2.0), point2(7.0, 4.0));
        let der = vec2(0.0, 0.0);
        assert_cc!(der, segment.der2(0.0));
        assert_cc!(der, segment.der2(0.5));
        assert_cc!(der, segment.der2(1.0));
    }

    #[test]
    fn tangent() {
        let segment = segment(point2(2.0, 2.0), point2(7.0, 4.0));
        let der = vec2(5.0, 2.0).normalize();
        assert_cc!(der, segment.tangent(0.0));
        assert_cc!(der, segment.tangent(0.5));
        assert_cc!(der, segment.tangent(1.0));
    }

    #[test]
    fn normal() {
        let segment = segment(point2(2.0, 2.0), point2(7.0, 4.0));
        let der = vec2(5.0, 2.0).normalize().orthogonal();
        assert_cc!(der, segment.normal(0.0));
        assert_cc!(der, segment.normal(0.5));
        assert_cc!(der, segment.normal(1.0));
    }

    #[test]
    fn local_axes() {
        let segment = segment(point2(2.0, 2.0), point2(7.0, 4.0));

        let (x_axis, y_axis) = segment.local_axes(0.0);
        assert_cc!(vec2(5.0, 2.0).normalize(), y_axis);
        assert_cc!(vec2(2.0, -5.0).normalize(), x_axis);
    }
}
