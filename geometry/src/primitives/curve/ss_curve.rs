use std::rc::Rc;

use common::SurfaceId;

use crate::{math::Point2, primitives::SurfaceSolver, IGeometry, PrimitiveGeometry};

use super::{ICurvePoint, ICurveSolver};

#[derive(Debug)]
pub struct SSCurveParams {
    u: f64,
    s0: Point2,
    s1: Point2,
}

#[derive(Clone, Debug)]
pub struct SSCurve {
    s0: SurfaceId,
    s1: SurfaceId,
    points: Rc<Vec<SSCurveParams>>,
}
impl SSCurve {
    pub fn new(s0: SurfaceId, s1: SurfaceId, points: Vec<SSCurveParams>) -> Self {
        Self {
            s0,
            s1,
            points: Rc::new(points),
        }
    }

    pub fn solver(&self, geometry: &PrimitiveGeometry) -> SSCurveSolver {
        SSCurveSolver::new(
            geometry.surface_solver(self.s0).unwrap(),
            geometry.surface_solver(self.s1).unwrap(),
            self.points.clone(),
        )
    }
}

#[derive(Clone, Debug)]
pub struct SSCurveSolver {
    inner: Rc<SSCurveSolverInner>,
}
impl SSCurveSolver {
    fn new(s0: SurfaceSolver, s1: SurfaceSolver, points: Rc<Vec<SSCurveParams>>) -> Self {
        Self {
            inner: Rc::new(SSCurveSolverInner::new(s0, s1, points)),
        }
    }
}
impl ICurveSolver for SSCurveSolver {
    type Point = SSCurvePoint;

    fn domain(&self) -> (f64, f64) {
        todo!()
    }

    fn point(&self, u: f64) -> Self::Point {
        todo!()
    }

    fn never_tangent(&self) -> &crate::math::Vec3 {
        todo!()
    }
}

#[derive(Debug)]
struct SSCurveSolverInner {
    s0: SurfaceSolver,
    s1: SurfaceSolver,
    points: Rc<Vec<SSCurveParams>>,
}
impl SSCurveSolverInner {
    pub fn new(s0: SurfaceSolver, s1: SurfaceSolver, points: Rc<Vec<SSCurveParams>>) -> Self {
        Self { s0, s1, points }
    }
}

pub struct SSCurvePoint {
    inner: Rc<SSCurvePointInner>,
}
impl SSCurvePoint {
    pub fn new(ss_curve: SSCurveSolver, u: f64) -> Self {
        Self { inner: todo!() }
    }
}
impl ICurvePoint for SSCurvePoint {
    fn u(&self) -> f64 {
        todo!()
    }

    fn pos(&self) -> &crate::math::Point3 {
        todo!()
    }

    fn der1(&self) -> &crate::math::Vec3 {
        todo!()
    }

    fn der2(&self) -> &crate::math::Vec3 {
        todo!()
    }

    fn der3(&self) -> &crate::math::Vec3 {
        todo!()
    }
}

struct SSCurvePointInner {
    u: f64,
    ss_curve: SSCurveSolver,
}
impl SSCurvePointInner {
    fn new(ss_curve: SSCurveSolver, u: f64) -> Self {
        Self { u, ss_curve }
    }
}
