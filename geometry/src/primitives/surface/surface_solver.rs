use super::{ISurfacePoint, SurfacePoint, SweepSolver};
use crate::{
    math::{deg, richardson_extrapolate, vec2, Coincidence, Mat22, Scalar, Vec2, Vec3},
    primitives::curve::CurveSolver,
    tessellate::{BspTree, TessellatedSurface, TessellationTolerance},
};
use std::{cell::OnceCell, rc::Rc};

pub trait ISurfaceSolver<'a, S: Scalar> {
    type Point: ISurfacePoint<S>;

    fn domain(&self) -> (Vec2<S>, Vec2<S>);

    fn domain_span(&self) -> Vec2<S> {
        let (min, max) = self.domain();
        max - min
    }

    fn point(&'a self, uv: Vec2<S>) -> Self::Point;
}

#[derive(Debug, Clone)]
pub struct PointToSurfaceProjection<S: Scalar> {
    pub iter: u32,
    pub uv: Vec2<S>,
    pub pos: Vec3<S>,
    pub du: Vec3<S>,
    pub dv: Vec3<S>,
    pub diff: Vec3<S>,
    pub dist: S,
}

#[derive(Debug)]
pub enum SurfaceSolverKind<S: Scalar> {
    Sweep(SweepSolver<S>),
}
impl<S: Scalar> From<SweepSolver<S>> for SurfaceSolverKind<S> {
    fn from(solver: SweepSolver<S>) -> Self {
        Self::Sweep(solver)
    }
}

#[derive(Debug)]
pub struct SurfaceSolver<S: Scalar> {
    kind: SurfaceSolverKind<S>,
    is_closed_u: OnceCell<bool>,
    is_closed_v: OnceCell<bool>,
    projection_bsp: Rc<OnceCell<BspTree>>,
}
impl<S: Scalar> SurfaceSolver<S> {
    pub(super) fn new(kind: SurfaceSolverKind<S>) -> Self {
        Self {
            kind,
            is_closed_u: OnceCell::new(),
            is_closed_v: OnceCell::new(),
            projection_bsp: Rc::new(OnceCell::new()),
        }
    }

    pub fn sweep(profile: CurveSolver<S>, path: CurveSolver<S>) -> Self {
        SweepSolver::new(profile, path).into()
    }

    pub fn domain(&self) -> (Vec2<S>, Vec2<S>) {
        match &self.kind {
            SurfaceSolverKind::Sweep(sweep) => sweep.domain(),
        }
    }

    pub fn point(&self, uv: Vec2<S>) -> SurfacePoint<S> {
        match &self.kind {
            SurfaceSolverKind::Sweep(sweep) => SurfacePoint::from(sweep.point(uv)),
        }
    }

    pub fn is_closed_u(&self) -> bool {
        *self.is_closed_u.get_or_init(|| {
            let (Vec2 { x: u_min, y: v_min }, Vec2 { x: u_max, y: v_max }) = self.domain();
            let v_mid = (v_min + v_max) / S::TWO;
            let start = self.point(vec2(u_min, v_mid));
            let end = self.point(vec2(u_max, v_mid));
            let (start_du, _) = start.der1();
            let (end_du, _) = end.der1();
            start.pos().cc(*end.pos()) && start_du.normalize().cc(end_du.normalize())
        })
    }

    pub fn is_closed_v(&self) -> bool {
        *self.is_closed_v.get_or_init(|| {
            let (Vec2 { x: u_min, y: v_min }, Vec2 { x: u_max, y: v_max }) = self.domain();
            let u_mid = (u_min + u_max) / S::TWO;
            let start = self.point(vec2(u_mid, v_min));
            let end = self.point(vec2(u_mid, v_max));
            let (_, start_dv) = start.der1();
            let (_, end_dv) = end.der1();
            start.pos().cc(*end.pos()) && start_dv.normalize().cc(end_dv.normalize())
        })
    }

    /*
    pub fn project_point(&self, point: Vec3<S>) -> Vec<PointToSurfaceProjection<S>> {
        let initial_guesses = self.projection_starting_params(point, true, true);
        let mut results = vec![];

        for guess in initial_guesses {
            if let Some(res) = self.project_from_starting_param(point, guess) {
                // Singularities can cause false positive results. We filter these out
                // by using estimated tangents for any zero derivatives and checking
                // that the point really is perpendicualr to these.
                let (tangent_u, tangent_v) = {
                    let tangent_u = if res.du.cc(Vec3::ZERO) {
                        match self.est_tangent_u(res.uv) {
                            Some(tan) => tan,
                            None => {
                                continue;
                            }
                        }
                    } else {
                        res.du.normalize()
                    };

                    let tangent_v = if res.dv.cc(Vec3::ZERO) {
                        match self.est_tangent_v(res.uv) {
                            Some(tan) => tan,
                            None => {
                                continue;
                            }
                        }
                    } else {
                        res.dv.normalize()
                    };

                    (tangent_u, tangent_v)
                };

                if tangent_u.dot(res.diff).cc(S::ZERO) && tangent_v.dot(res.diff).cc(S::ZERO) {
                    results.push(res);
                }
            }
        }

        results.sort_by(|a, b| a.dist.total_cmp(&b.dist));

        results
    }

    fn projection_bsp(&self) -> &BspTree {
        self.projection_bsp.get_or_init(|| {
            TessellatedSurface::create_bsp(self, &TessellationTolerance::Angle(deg(5.0)))
        })
    }

    pub fn projection_starting_params(
        &self,
        p: Vec3<S>,
        allow_above_focal_point: bool,
        allow_below_focal_point: bool,
    ) -> Vec<Vec2<S>> {
        let mut start_params = vec![];

        let bsp = self.projection_bsp();

        bsp.visit_spaces(&mut |n: f64, s: f64, w: f64, e: f64| {
            let nw = self.point(vec2(w, n));
            let ne = self.point(vec2(e, n));
            let sw = self.point(vec2(w, s));
            let se = self.point(vec2(e, s));

            let perp_to_n = || {
                Self::is_perpendicular(
                    p,
                    *nw.pos(),
                    nw.der1().0,
                    *ne.pos(),
                    ne.der1().0,
                    allow_above_focal_point,
                    allow_below_focal_point,
                )
            };

            let perp_to_s = || {
                Self::is_perpendicular(
                    p,
                    *sw.pos(),
                    sw.der1().0,
                    *se.pos(),
                    se.der1().0,
                    allow_above_focal_point,
                    allow_below_focal_point,
                )
            };

            let perp_to_w = || {
                Self::is_perpendicular(
                    p,
                    *sw.pos(),
                    sw.der1().1,
                    *nw.pos(),
                    nw.der1().1,
                    allow_above_focal_point,
                    allow_below_focal_point,
                )
            };

            let perp_to_e = || {
                Self::is_perpendicular(
                    p,
                    *se.pos(),
                    se.der1().1,
                    *ne.pos(),
                    ne.der1().1,
                    allow_above_focal_point,
                    allow_below_focal_point,
                )
            };

            if (perp_to_e() || perp_to_w()) && (perp_to_n() || perp_to_s()) {
                start_params.push(vec2((w + e) / S::TWO, (s + n) / S::TWO));
            }
        });

        start_params
    }

    fn is_perpendicular(
        p: Vec3<S>,
        p0_pos: Vec3<S>,
        p0_d1: Vec3<S>,
        p1_pos: Vec3<S>,
        p1_d1: Vec3<S>,
        allow_above_focal_point: bool,
        allow_below_focal_point: bool,
    ) -> bool {
        let p0_p = p - p0_pos;
        let p_p1 = p1_pos - p;

        let r1 = p0_p.dot(p0_d1);
        let r2 = p_p1.dot(p1_d1);

        // perpendicular at p0 or p1
        (r1 == S::ZERO || r2 == S::ZERO) ||
                // perpendicular from outside of curve or inside "focal point"
                (allow_above_focal_point && r1 > S::ZERO && r2 > S::ZERO) ||
                // perpendicular from below curve beyond the "focal point"
                (allow_below_focal_point && r1 < S::ZERO && r2 < S::ZERO)
    }

    fn project_from_starting_param(
        &self,
        point: Vec3<S>,
        mut uv: Vec2<S>,
    ) -> Option<PointToSurfaceProjection<S>> {
        const MAX_ITER: u32 = 32;

        let (Vec2 { x: u_min, y: v_min }, Vec2 { x: u_max, y: v_max }) = self.domain();

        for iter in 0..MAX_ITER {
            let cp = self.point(uv);

            let pos = *cp.pos();
            let (du, dv) = *cp.der1();
            let (duu, duv, dvv) = *cp.der2();
            let diff = pos - point;
            let dist = diff.magnitude();

            let jacobian_off_diagonal = du.dot(dv) + diff.dot(duv);

            let jacobian_inverse = Mat22::new(
                du.magnitude2() + diff.dot(duu),
                jacobian_off_diagonal,
                jacobian_off_diagonal,
                dv.magnitude2() + diff.dot(dvv),
            )
            .inverse()
            .unwrap();

            let k = -vec2(diff.dot(du), diff.dot(dv));

            let delta = jacobian_inverse * k;

            let result = Some(PointToSurfaceProjection {
                iter,
                uv,
                du,
                dv,
                pos,
                diff,
                dist,
            });

            // Some stopping conditions
            {
                // Point coincidence
                if dist.cc_newton(0.0) {
                    return result;
                }

                // Zero cosine
                if (du.dot(diff) / (du.magnitude() * dist)).cc_newton(0.0)
                    && (dv.dot(diff) / (dv.magnitude() * dist)).cc_newton(0.0)
                {
                    return result;
                }
            }

            // Get the next parameter value, making sure it stays in the domain
            let u_next = uv.u() + delta.u();
            let u_next = if self.is_closed_u() {
                if u_next < u_min {
                    u_max - (u_min - u_next)
                } else if u_next > u_max {
                    u_min + (u_next - u_max)
                } else {
                    u_next
                }
            } else {
                u_next.clamp(u_min, u_max)
            };

            let v_next = uv.v() + delta.v();
            let v_next = if self.is_closed_v() {
                if v_next < v_min {
                    v_max - (v_min - v_next)
                } else if v_next > v_max {
                    v_min + (v_next - v_max)
                } else {
                    v_next
                }
            } else {
                v_next.clamp(v_min, v_max)
            };

            let uv_next = vec2(u_next, v_next);

            // Additional stopping condition: parameters haven't changed significantly or
            // are off the end of the curve (if unclosed).
            if ((uv_next.u() - uv.u()) * du + (uv_next.v() - uv.v()) * dv)
                .magnitude()
                .cc_newton(0.0)
            {
                return result;
            }

            uv = uv_next;
        }

        None
    }
    */

    pub fn est_tangent_u(&self, uv: Vec2<S>) -> Option<Vec3<S>> {
        panic!();
        /*
        let (Vec2 { y: v_min, .. }, Vec2 { y: v_max, .. }) = self.domain();
        let start_dist = S::ONE / S::exact(10.0);

        let end_v = uv.v();
        let start_v = {
            let dist_to_max = (v_max - end_v).abs();
            let dist_to_min = (v_min - end_v).abs();

            if dist_to_max < dist_to_min {
                // If closer to top of U range, start from below
                end_v - start_dist
            } else {
                // Otherwise start from above
                end_v + start_dist
            }
        };

        richardson_extrapolate(
            |v: S| {
                let point = self.point(vec2(uv.u(), v));
                let (du, _) = point.der1();
                du.normalize()
            },
            |a, b| (a - b).magnitude(),
            start_v,
            end_v,
            40,
            1e-10,
            //COINCIDENT_TOL,
        )
         */
    }

    pub fn est_tangent_v(&self, uv: Vec2<S>) -> Option<Vec3<S>> {
        panic!();
        /*
        let (Vec2 { x: u_min, .. }, Vec2 { x: u_max, .. }) = self.domain();
        let start_dist = S::ONE / S::exact(10.0);

        let end_u = uv.u();
        let start_u = {
            let dist_to_max = (u_max - end_u).abs();
            let dist_to_min = (u_min - end_u).abs();

            if dist_to_max < dist_to_min {
                // If closer to top of U range, start from below
                end_u - start_dist
            } else {
                // Otherwise start from above
                end_u + start_dist
            }
        };

        richardson_extrapolate(
            |u: S| {
                let point = self.point(vec2(u, uv.v()));
                let (_, dv) = point.der1();
                dv.normalize()
            },
            |a, b| (a - b).magnitude(),
            start_u,
            end_u,
            40,
            1e-10,
            //COINCIDENT_TOL,
        )
         */
    }
}
impl<S: Scalar> From<SweepSolver<S>> for SurfaceSolver<S> {
    fn from(sweep: SweepSolver<S>) -> Self {
        Self::new(SurfaceSolverKind::Sweep(sweep))
    }
}
