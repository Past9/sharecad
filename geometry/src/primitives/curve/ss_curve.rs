use std::{cell::OnceCell, rc::Rc};

use common::SurfaceId;

use crate::{
    math::{point2, vec4, Point2, Point3, Vec4},
    primitives::{ISurfacePoint, SurfaceIntersectionTransversal, SurfaceSolver},
    IGeometry, PrimitiveGeometry,
};

use super::{ICurvePoint, ICurveSolver};

#[derive(Debug, Clone)]
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
        (
            self.inner.points[0].u,
            self.inner.points[self.inner.points.len() - 1].u,
        )
    }

    fn point(&self, u: f64) -> Self::Point {
        // TODO make this faster with a binary search or BTree
        for i in 0..self.inner.points.len() - 1 {
            let cur = &self.inner.points[i];
            let next = &self.inner.points[i + 1];

            if cur.u < u && next.u > u {
                let intersection =
                    SurfaceIntersectionTransversal::new(&self.inner.s0, &self.inner.s1);
                let Vec4 { x, y, z, w } =
                    intersection.rk_step(vec4(cur.s0.x, cur.s0.y, cur.s1.x, cur.s1.y), u - cur.u);
                return SSCurvePoint::new(
                    self.clone(),
                    SSCurveParams {
                        u,
                        s0: point2(x, y),
                        s1: point2(z, w),
                    },
                );
            } else if cur.u == u {
                return SSCurvePoint::new(self.clone(), cur.clone());
            } else if next.u == u {
                return SSCurvePoint::new(self.clone(), next.clone());
            }
        }

        panic!("No sample point")
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
    pub fn new(ss_curve: SSCurveSolver, params: SSCurveParams) -> Self {
        Self {
            inner: Rc::new(SSCurvePointInner::new(ss_curve, params)),
        }
    }
}
impl ICurvePoint for SSCurvePoint {
    fn u(&self) -> f64 {
        self.inner.params.u
    }

    fn pos(&self) -> &crate::math::Point3 {
        self.inner.pos.get_or_init(|| {
            let s0_point = *self
                .inner
                .ss_curve
                .inner
                .s0
                .point(self.inner.params.s0)
                .pos();
            let s1_point = *self
                .inner
                .ss_curve
                .inner
                .s1
                .point(self.inner.params.s1)
                .pos();

            (s0_point + s1_point) / 2.0
        })
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
    params: SSCurveParams,
    ss_curve: SSCurveSolver,

    pos: OnceCell<Point3>,
    der1: OnceCell<Point3>,
    der2: OnceCell<Point3>,
}
impl SSCurvePointInner {
    fn new(ss_curve: SSCurveSolver, params: SSCurveParams) -> Self {
        Self {
            ss_curve,
            params,
            pos: OnceCell::new(),
            der1: OnceCell::new(),
            der2: OnceCell::new(),
        }
    }
}
