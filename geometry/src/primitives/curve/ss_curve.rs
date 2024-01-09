use std::{cell::OnceCell, rc::Rc};

use common::SurfaceId;

use crate::{
    math::{point2, vec4, Point2, Point3, Vec3, Vec4},
    primitives::{ISurfacePoint, SSCurveSampler, SurfaceSolver},
    IGeometry, PrimitiveGeometry,
};

use super::{ICurvePoint, ICurveSolver};

#[derive(Debug, Clone)]
pub struct SSCurveParams {
    pub u: f64,
    pub pos: Point3,
    pub s0: Point2,
    pub s1: Point2,
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
                /*
                let intersection = SSCurveSampler::new_from_starting_params(
                    &self.inner.s0,
                    &self.inner.s1,
                    self.inner.points[0].clone(),
                );
                 */
                //let next = intersection.next(cur.clone(), u - cur.u).unwrap();
                let next = SSCurveSampler::rk_step(&self.inner.s0, &self.inner.s1, cur, u - cur.u);

                return SSCurvePoint::new(self.clone(), next);
            } else if cur.u == u {
                return SSCurvePoint::new(self.clone(), cur.clone());
            } else if next.u == u {
                return SSCurvePoint::new(self.clone(), next.clone());
            }
        }

        panic!("No sample point at {}", u)
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
        self.inner.der1.get_or_init(|| {
            let s0_point = self.inner.ss_curve.inner.s0.point(self.inner.params.s0);
            let s1_point = self.inner.ss_curve.inner.s1.point(self.inner.params.s1);

            let (s0_du, s0_dv) = *s0_point.der1();
            let (s1_du, s1_dv) = *s1_point.der1();

            let np = s0_du.cross(s0_dv);
            let nq = s1_du.cross(s1_dv);

            let d1 = np.cross(nq); //.normalize();

            d1
        })
    }

    fn der2(&self) -> &crate::math::Vec3 {
        self.inner.der2.get_or_init(|| {
            let s0_point = self.inner.ss_curve.inner.s0.point(self.inner.params.s0);
            let s1_point = self.inner.ss_curve.inner.s1.point(self.inner.params.s1);

            let (s0_du, s0_dv) = *s0_point.der1();
            let (s1_du, s1_dv) = *s1_point.der1();

            let (s0_duu, _, s0_dvv) = *s0_point.der2();
            let (s1_duu, _, s1_dvv) = *s1_point.der2();

            let np = s0_du.cross(s0_dv);
            let nq = s1_du.cross(s1_dv);

            let np_p = s0_duu.cross(s0_dv) + s0_du.cross(s0_dvv);
            let nq_p = s1_duu.cross(s1_dv) + s1_du.cross(s1_dvv);

            let r = np.cross(nq);
            let r_p = np_p.cross(nq) + np.cross(nq_p);

            return r_p;

            let m = r.dot(r);
            let m_p = r_p.dot(r) + r.dot(r_p);

            let d2 = (r_p * m - r * m_p) / m.powi(2);

            d2

            /*
            let d1_num = np.cross(nq);
            let d1_den = d1_num.magnitude();

            let d1_num_der1 = np_d.cross(nq) + np.cross(nq_d);
            let d1_den_der1 = np_d.dot(nq) + np.dot(nq_d);``
             */
        })
    }

    fn der3(&self) -> &crate::math::Vec3 {
        todo!()
    }

    fn curvature(&self) -> f64 {
        let k = (self.der1().magnitude().powi(3) / (self.der1().cross(*self.der2())).magnitude())
            .recip();

        k
    }
}

struct SSCurvePointInner {
    params: SSCurveParams,
    ss_curve: SSCurveSolver,

    pos: OnceCell<Point3>,
    der1: OnceCell<Vec3>,
    der2: OnceCell<Vec3>,
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
