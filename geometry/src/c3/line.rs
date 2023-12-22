use std::cell::OnceCell;

use space::{vec3, Coincidence, Point3, Vec3};

use crate::{ICurve, ICurvePoint};

pub struct LineCurve {
    start: Point3,
    end: Point3,
}
impl LineCurve {
    pub fn new(start: Point3, end: Point3) -> Self {
        Self { start, end }
    }
}
impl<'a> ICurve<'a> for LineCurve {
    type Point = LinePoint<'a>;

    fn domain(&self) -> (f64, f64) {
        (0.0, 1.0)
    }

    fn point(&'a self, u: f64) -> Self::Point {
        LinePoint::new(self, u)
    }
}

pub struct LinePoint<'a> {
    u: f64,
    line: &'a LineCurve,

    eval: OnceCell<Point3>,
    der1: OnceCell<Vec3>,
    der2: OnceCell<Vec3>,
    der3: OnceCell<Vec3>,
    never_tangent: OnceCell<Vec3>,
}
impl<'a> LinePoint<'a> {
    pub fn new(line: &'a LineCurve, u: f64) -> Self {
        Self {
            line,
            u,

            eval: OnceCell::new(),
            der1: OnceCell::new(),
            der2: OnceCell::new(),
            der3: OnceCell::new(),
            never_tangent: OnceCell::new(),
        }
    }
}
impl<'a> ICurvePoint for LinePoint<'a> {
    fn u(&self) -> f64 {
        self.u
    }

    fn eval(&self) -> &Point3 {
        self.eval
            .get_or_init(|| ((1.0 - self.u) * self.line.start + self.u * self.line.end))
    }

    fn der1(&self) -> &Vec3 {
        self.der1.get_or_init(|| self.line.end - self.line.start)
    }

    fn der2(&self) -> &Vec3 {
        self.der2.get_or_init(|| Vec3::ZERO)
    }

    fn der3(&self) -> &Vec3 {
        self.der3.get_or_init(|| Vec3::ZERO)
    }

    fn never_tangent(&self) -> &Vec3 {
        self.never_tangent.get_or_init(|| {
            let tangent = (self.line.end - self.line.start).normalize();
            if tangent.z.abs().cc(1.0) {
                vec3(0.0, tangent.z, 0.0)
            } else {
                vec3(-tangent.y, tangent.x, tangent.z)
            }
        })
    }
}
