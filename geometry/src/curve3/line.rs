use space::{vec3, Coincidence, Point3, Vec3};

use crate::Curve3Impl;

#[derive(Debug, Clone)]
pub struct Line {
    start: Point3,
    end: Point3,
}
impl Line {
    pub fn new(start: Point3, end: Point3) -> Self {
        Self {
            start: start,
            end: end,
        }
    }
}
impl Curve3Impl for Line {
    fn u_min(&self) -> f64 {
        0.0
    }

    fn u_max(&self) -> f64 {
        1.0
    }

    fn eval(&self, u: f64) -> Point3 {
        (1.0 - u) * self.start + u * self.end
    }

    fn der1(&self, _u: f64) -> Vec3 {
        self.end - self.start
    }

    fn der2(&self, _u: f64) -> Vec3 {
        Vec3::ZERO
    }

    fn der3(&self, _u: f64) -> Vec3 {
        Vec3::ZERO
    }

    fn period(&self) -> Option<f64> {
        None
    }

    fn never_tangent(&self) -> Vec3 {
        let tangent = (self.end - self.start).normalize();
        if tangent.z.abs().cc(1.0) {
            vec3(0.0, tangent.z, 0.0)
        } else {
            vec3(-tangent.y, tangent.x, tangent.z)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{test::validate_ders_curve, Curve3Impl, Line};
    use space::{assert_cc, point3};

    fn test_line() -> Line {
        Line::new(point3(1.0, 2.0, 3.0), point3(-3.0, -1.0, -2.0))
    }

    #[test]
    fn helix_points() {
        let points = test_line().eval_sections(8);

        assert_cc!(point3(1.0, 2.0, 3.0), points[0]);
        assert_cc!(point3(0.5, 1.625, 2.375), points[1]);
        assert_cc!(point3(0.0, 1.25, 1.75), points[2]);
        assert_cc!(point3(-0.5, 0.875, 1.125), points[3]);
        assert_cc!(point3(-1.0, 0.5, 0.5), points[4]);
        assert_cc!(point3(-1.5, 0.125, -0.125), points[5]);
        assert_cc!(point3(-2.0, -0.25, -0.75), points[6]);
        assert_cc!(point3(-2.5, -0.625, -1.375), points[7]);
        assert_cc!(point3(-3.0, -1.0, -2.0), points[8]);
    }

    #[test]
    fn line_validate_ders() {
        validate_ders_curve(&test_line(), 100, 1e-7);
    }
}
