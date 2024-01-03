use crate::{geometry, IGeometry, PrimitiveGeometry};

use super::{CurvePointAxes, ICurvePoint, ICurveSolver};
use common::PointId;
use space::{vec3, Coincidence, Point3, Vec3};
use std::cell::OnceCell;

#[derive(Clone, Debug)]
pub struct Line {
    start: PointId,
    end: PointId,
}
impl Line {
    pub fn new(start: PointId, end: PointId) -> Self {
        Self { start, end }
    }

    pub fn solver(&self, geometry: &PrimitiveGeometry) -> LineSolver {
        LineSolver {
            start: geometry.point(self.start).unwrap().to_owned(),
            end: geometry.point(self.end).unwrap().to_owned(),
            never_tangent: OnceCell::new(),
        }
    }
}

#[derive(Clone)]
pub struct LineSolver {
    start: Point3,
    end: Point3,

    never_tangent: OnceCell<Vec3>,
}
impl LineSolver {
    pub fn new(start: Point3, end: Point3) -> Self {
        Self {
            start,
            end,
            never_tangent: OnceCell::new(),
        }
    }
}
impl<'a> ICurveSolver<'a> for LineSolver {
    type Point = LinePoint<'a>;

    fn domain(&self) -> (f64, f64) {
        (0.0, 1.0)
    }

    fn point(&'a self, u: f64) -> Self::Point {
        LinePoint::new(self, u)
    }

    fn never_tangent(&self) -> &Vec3 {
        self.never_tangent.get_or_init(|| {
            let tangent = (self.end - self.start).normalize();
            if tangent.z.abs().cc(1.0) {
                vec3(0.0, tangent.z, 0.0)
            } else {
                vec3(-tangent.y, tangent.x, tangent.z)
            }
        })
    }
}

pub struct LinePoint<'a> {
    u: f64,
    line: &'a LineSolver,

    eval: OnceCell<Point3>,
    der1: OnceCell<Vec3>,
    der2: OnceCell<Vec3>,
    der3: OnceCell<Vec3>,
    axes: OnceCell<CurvePointAxes<'a>>,
}
impl<'a> LinePoint<'a> {
    pub fn new(line: &'a LineSolver, u: f64) -> Self {
        Self {
            line,
            u,

            eval: OnceCell::new(),
            der1: OnceCell::new(),
            der2: OnceCell::new(),
            der3: OnceCell::new(),
            axes: OnceCell::new(),
        }
    }

    fn axes(&'a self) -> &CurvePointAxes<'a> {
        self.axes
            .get_or_init(|| CurvePointAxes::new(self, *self.line.never_tangent()))
    }
}
impl<'a> ICurvePoint<'a> for LinePoint<'a> {
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

    fn axes(&'a self) -> &space::Mat33 {
        self.axes().axes_mat()
    }

    fn axes_der1(&'a self) -> &space::Mat33 {
        self.axes().axes_der1_mat()
    }

    fn axes_der2(&'a self) -> &space::Mat33 {
        self.axes().axes_der2_mat()
    }
}
