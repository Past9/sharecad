use super::{ICurvePoint, ICurveSolver};
use crate::{
    math::{vec3, Coincidence, Point3, Vec3},
    IGeometry, PrimitiveGeometry,
};
use common::PointId;
use std::{cell::OnceCell, rc::Rc};

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

#[derive(Clone, Debug)]
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
impl ICurveSolver for LineSolver {
    type Point = LinePoint;

    fn domain(&self) -> (f64, f64) {
        (0.0, 1.0)
    }

    fn point(&self, u: f64) -> Self::Point {
        LinePoint::new(self.clone(), u)
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

pub struct LinePoint {
    inner: Rc<LinePointInner>,
}
impl LinePoint {
    pub fn new(line: LineSolver, u: f64) -> Self {
        Self {
            inner: Rc::new(LinePointInner::new(line, u)),
        }
    }
}

struct LinePointInner {
    u: f64,
    line: LineSolver,

    eval: OnceCell<Point3>,
    der1: OnceCell<Vec3>,
    der2: OnceCell<Vec3>,
    der3: OnceCell<Vec3>,
}
impl LinePointInner {
    pub fn new(line: LineSolver, u: f64) -> Self {
        Self {
            line,
            u,

            eval: OnceCell::new(),
            der1: OnceCell::new(),
            der2: OnceCell::new(),
            der3: OnceCell::new(),
        }
    }
}
impl ICurvePoint for LinePoint {
    fn u(&self) -> f64 {
        self.inner.u
    }

    fn pos(&self) -> &Point3 {
        self.inner.eval.get_or_init(|| {
            (1.0 - self.inner.u) * self.inner.line.start + self.inner.u * self.inner.line.end
        })
    }

    fn der1(&self) -> &Vec3 {
        self.inner
            .der1
            .get_or_init(|| self.inner.line.end - self.inner.line.start)
    }

    fn der2(&self) -> &Vec3 {
        self.inner.der2.get_or_init(|| Vec3::ZERO)
    }

    fn der3(&self) -> &Vec3 {
        self.inner.der3.get_or_init(|| Vec3::ZERO)
    }

    fn curvature(&self) -> f64 {
        0.0
    }
}
