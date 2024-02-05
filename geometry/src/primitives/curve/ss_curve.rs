use std::{cell::OnceCell, rc::Rc};

use common::SurfaceId;

use crate::{
    math::{vec4, Scalar, Vec2, Vec3, Vec4},
    primitives::{ISurfacePoint, SSCurveSampler, SurfaceSolver},
    IGeometry, PrimitiveGeometry,
};

use super::{ICurvePoint, ICurveSolver};

#[derive(Debug, Clone)]
pub struct SSCurveParams<S: Scalar> {
    pub u: S,
    pub pos: Vec3<S>,
    pub s0: Vec2<S>,
    pub s1: Vec2<S>,
}

#[derive(Clone, Debug)]
pub struct SSCurve<S: Scalar> {
    s0: SurfaceId,
    s1: SurfaceId,
    points: Rc<Vec<SSCurveParams<S>>>,
}
impl<S: Scalar> SSCurve<S> {
    pub fn new(s0: SurfaceId, s1: SurfaceId, points: Vec<SSCurveParams<S>>) -> Self {
        Self {
            s0,
            s1,
            points: Rc::new(points),
        }
    }

    pub fn solver(&self, geometry: &PrimitiveGeometry<S>) -> SSCurveSolver<S> {
        SSCurveSolver::new(
            geometry.surface_solver(self.s0).unwrap(),
            geometry.surface_solver(self.s1).unwrap(),
            self.points.clone(),
        )
    }
}

#[derive(Clone, Debug)]
pub struct SSCurveSolver<S: Scalar> {
    inner: Rc<SSCurveSolverInner<S>>,
}
impl<S: Scalar> SSCurveSolver<S> {
    fn new(s0: SurfaceSolver<S>, s1: SurfaceSolver<S>, points: Rc<Vec<SSCurveParams<S>>>) -> Self {
        Self {
            inner: Rc::new(SSCurveSolverInner::new(s0, s1, points)),
        }
    }
}
impl<S: Scalar> ICurveSolver<S> for SSCurveSolver<S> {
    type PointSolver = SSCurvePoint<S>;

    fn domain(&self) -> (S, S) {
        (
            self.inner.points[0].u,
            self.inner.points[self.inner.points.len() - 1].u,
        )
    }

    fn point(&self, u: S) -> Self::PointSolver {
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

    fn never_tangent(&self) -> &Vec3<S> {
        todo!()
    }
}

#[derive(Debug)]
struct SSCurveSolverInner<S: Scalar> {
    s0: SurfaceSolver<S>,
    s1: SurfaceSolver<S>,
    points: Rc<Vec<SSCurveParams<S>>>,
}
impl<S: Scalar> SSCurveSolverInner<S> {
    pub fn new(
        s0: SurfaceSolver<S>,
        s1: SurfaceSolver<S>,
        points: Rc<Vec<SSCurveParams<S>>>,
    ) -> Self {
        Self { s0, s1, points }
    }
}

pub struct SSCurvePoint<S: Scalar> {
    inner: Rc<SSCurvePointInner<S>>,
}
impl<S: Scalar> SSCurvePoint<S> {
    pub fn new(ss_curve: SSCurveSolver<S>, params: SSCurveParams<S>) -> Self {
        Self {
            inner: Rc::new(SSCurvePointInner::new(ss_curve, params)),
        }
    }
}
impl<S: Scalar> ICurvePoint<S> for SSCurvePoint<S> {
    fn u(&self) -> S {
        self.inner.params.u
    }

    fn pos(&self) -> &Vec3<S> {
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

            (s0_point + s1_point) / S::TWO
        })
    }

    fn der1(&self) -> &Vec3<S> {
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

    fn der2(&self) -> &Vec3<S> {
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

    fn der3(&self) -> &Vec3<S> {
        todo!()
    }

    fn curvature(&self) -> S {
        let k = (self.der1().magnitude().powi(3) / (self.der1().cross(*self.der2())).magnitude())
            .recip();

        k
    }
}

struct SSCurvePointInner<S: Scalar> {
    params: SSCurveParams<S>,
    ss_curve: SSCurveSolver<S>,

    pos: OnceCell<Vec3<S>>,
    der1: OnceCell<Vec3<S>>,
    der2: OnceCell<Vec3<S>>,
}
impl<S: Scalar> SSCurvePointInner<S> {
    fn new(ss_curve: SSCurveSolver<S>, params: SSCurveParams<S>) -> Self {
        Self {
            ss_curve,
            params,
            pos: OnceCell::new(),
            der1: OnceCell::new(),
            der2: OnceCell::new(),
        }
    }
}
