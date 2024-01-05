use super::{ISurfacePoint, SurfacePoint, SweepSolver};
use crate::{
    math::{point2, vec2, Coincidence, Mat22, Point2, Point3, Vec2, Vec3},
    primitives::curve::CurveSolver,
};
use std::cell::OnceCell;

pub trait ISurfaceSolver<'a> {
    type Point: ISurfacePoint;

    fn domain(&self) -> (Point2, Point2);

    fn domain_span(&self) -> Vec2 {
        let (min, max) = self.domain();
        max - min
    }

    fn point(&'a self, uv: Point2) -> Self::Point;
}

#[derive(Debug, Clone)]
pub struct PointToSurfaceProjection {
    pub iter: u32,
    pub uv: Point2,
    pub pos: Point3,
    pub diff: Vec3,
    pub dist: f64,
}

pub enum SurfaceSolverKind {
    Sweep(SweepSolver),
}
impl From<SweepSolver> for SurfaceSolverKind {
    fn from(solver: SweepSolver) -> Self {
        Self::Sweep(solver)
    }
}

pub struct SurfaceSolver {
    kind: SurfaceSolverKind,
    is_closed_u: OnceCell<bool>,
    is_closed_v: OnceCell<bool>,
}
impl SurfaceSolver {
    pub(super) fn new(kind: SurfaceSolverKind) -> Self {
        Self {
            kind,
            is_closed_u: OnceCell::new(),
            is_closed_v: OnceCell::new(),
        }
    }

    pub fn sweep(profile: CurveSolver, path: CurveSolver) -> Self {
        SweepSolver::new(profile, path).into()
    }

    pub fn domain(&self) -> (Point2, Point2) {
        match &self.kind {
            SurfaceSolverKind::Sweep(sweep) => sweep.domain(),
        }
    }

    pub fn point(&self, uv: Point2) -> SurfacePoint {
        match &self.kind {
            SurfaceSolverKind::Sweep(sweep) => SurfacePoint::from(sweep.point(uv)),
        }
    }

    pub fn is_closed_u(&self) -> bool {
        *self.is_closed_u.get_or_init(|| {
            let (Point2 { x: u_min, y: v_min }, Point2 { x: u_max, y: v_max }) = self.domain();
            let v_mid = (v_min + v_max) / 2.0;
            let start = self.point(point2(u_min, v_mid));
            let end = self.point(point2(u_max, v_mid));
            let (start_du, _) = start.der1();
            let (end_du, _) = end.der1();
            start.pos().cc(*end.pos()) && start_du.normalize().cc(end_du.normalize())
        })
    }

    pub fn is_closed_v(&self) -> bool {
        *self.is_closed_v.get_or_init(|| {
            let (Point2 { x: u_min, y: v_min }, Point2 { x: u_max, y: v_max }) = self.domain();
            let u_mid = (u_min + u_max) / 2.0;
            let start = self.point(point2(u_mid, v_min));
            let end = self.point(point2(u_mid, v_max));
            let (_, start_dv) = start.der1();
            let (_, end_dv) = end.der1();
            start.pos().cc(*end.pos()) && start_dv.normalize().cc(end_dv.normalize())
        })
    }

    pub fn project_point(&self, point: Point3) -> Vec<PointToSurfaceProjection> {
        let initial_guesses = self.projection_starting_params(point, true, true);
        let mut results = vec![];

        for guess in initial_guesses {
            if let Some(res) = self.project_from_starting_param(point, guess) {
                results.push(res);
            }
        }

        results.sort_by(|a, b| a.dist.total_cmp(&b.dist));

        results
    }

    fn projection_starting_params(
        &self,
        p: Point3,
        allow_above_focal_point: bool,
        allow_below_focal_point: bool,
    ) -> Vec<Point2> {
        let (Point2 { x: u_min, y: v_min }, Point2 { x: u_max, y: v_max }) = self.domain();
        vec![point2((u_min + u_max) / 2.0, (v_min + v_max) / 2.0)]
    }

    fn project_from_starting_param(
        &self,
        point: Point3,
        mut uv: Point2,
    ) -> Option<PointToSurfaceProjection> {
        const MAX_ITER: u32 = 32;

        let (Point2 { x: u_min, y: v_min }, Point2 { x: u_max, y: v_max }) = self.domain();

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
                    || (dv.dot(diff) / (dv.magnitude() * dist)).cc_newton(0.0)
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

            let uv_next = point2(u_next, v_next);

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
}
impl From<SweepSolver> for SurfaceSolver {
    fn from(sweep: SweepSolver) -> Self {
        Self::new(SurfaceSolverKind::Sweep(sweep))
    }
}
