use std::cell::OnceCell;

use crate::{
    math::{deg, Angle, Coincidence, Point3, Quat, Vec3},
    tessellate::{TessellatedCurve, TessellationTolerance},
};

use super::{ArcSolver, CurvePoint, HelixSolver, ICurvePoint, LineSolver};

pub trait ICurveSolver<'a> {
    type Point: ICurvePoint<'a>;

    fn domain(&self) -> (f64, f64);

    fn domain_span(&self) -> f64 {
        let (min, max) = self.domain();
        max - min
    }

    fn point(&'a self, u: f64) -> Self::Point;

    fn never_tangent(&self) -> &Vec3;
}

#[derive(Clone, Debug)]
pub enum CurveSolverKind {
    Line(LineSolver),
    Arc(ArcSolver),
    Helix(HelixSolver),
}
impl From<LineSolver> for CurveSolverKind {
    fn from(solver: LineSolver) -> Self {
        Self::Line(solver)
    }
}
impl From<ArcSolver> for CurveSolverKind {
    fn from(solver: ArcSolver) -> Self {
        Self::Arc(solver)
    }
}
impl From<HelixSolver> for CurveSolverKind {
    fn from(solver: HelixSolver) -> Self {
        Self::Helix(solver)
    }
}

#[derive(Debug, Clone)]
pub struct PointToCurveProjection {
    pub iter: u32,
    pub u: f64,
    pub pos: Point3,
    pub diff: Vec3,
    pub dist: f64,
}

pub struct CurveSolver {
    kind: CurveSolverKind,
    is_closed: OnceCell<bool>,
    projection_tessellation: OnceCell<TessellatedCurve>,
}
impl CurveSolver {
    pub(super) fn new(kind: CurveSolverKind) -> Self {
        Self {
            kind,
            is_closed: OnceCell::new(),
            projection_tessellation: OnceCell::new(),
        }
    }

    pub fn line(start: Point3, end: Point3) -> Self {
        LineSolver::new(start, end).into()
    }

    pub fn arc(r: f64, angle: Angle, orientation: Quat, translation: Vec3) -> Self {
        ArcSolver::new(r, angle, orientation, translation).into()
    }

    pub fn helix(r: f64, h: f64, n: f64, orientation: Quat, translation: Vec3) -> Self {
        HelixSolver::new(r, h, n, orientation, translation).into()
    }

    pub fn domain(&self) -> (f64, f64) {
        match &self.kind {
            CurveSolverKind::Line(line) => line.domain(),
            CurveSolverKind::Helix(helix) => helix.domain(),
            CurveSolverKind::Arc(arc) => arc.domain(),
        }
    }

    pub fn point(&self, u: f64) -> CurvePoint {
        match &self.kind {
            CurveSolverKind::Line(line) => CurvePoint::from(line.point(u)),
            CurveSolverKind::Helix(helix) => CurvePoint::from(helix.point(u)),
            CurveSolverKind::Arc(arc) => CurvePoint::from(arc.point(u)),
        }
    }

    pub fn never_tangent(&self) -> &Vec3 {
        match &self.kind {
            CurveSolverKind::Line(line) => line.never_tangent(),
            CurveSolverKind::Helix(helix) => helix.never_tangent(),
            CurveSolverKind::Arc(arc) => arc.never_tangent(),
        }
    }

    /// Whether the curve is closed. Curves are considered closed when their starting and ending
    /// points and unit tangent vectors are the same.
    pub fn is_closed(&self) -> bool {
        *self.is_closed.get_or_init(|| {
            let (u_min, u_max) = self.domain();
            let start = self.point(u_min);
            let end = self.point(u_max);
            start.pos().cc(*end.pos()) && start.der1().normalize().cc(end.der1().normalize())
        })
    }

    pub fn invert_point(&self, point: Point3) -> Vec<PointToCurveProjection> {
        let initial_guesses = self.projection_starting_params(point, true, false);
        let mut results = vec![];

        for guess in initial_guesses {
            if let Some(res) = self.project_from_starting_param(point, guess) {
                if res.dist.cc(0.0) {
                    results.push(res);
                }
            }
        }

        results
    }

    pub fn project_point(&self, point: Point3) -> Vec<PointToCurveProjection> {
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

    fn projection_tessellation(&self) -> &TessellatedCurve {
        self.projection_tessellation
            .get_or_init(|| TessellatedCurve::create(self, &TessellationTolerance::Angle(deg(1.0))))
    }

    fn projection_starting_params(
        &self,
        p: Point3,
        allow_above_focal_point: bool,
        allow_below_focal_point: bool,
    ) -> Vec<f64> {
        let mut start_params = vec![];
        let samples = &self.projection_tessellation().points;

        for i in 1..samples.len() {
            let p0 = &samples[i - 1];
            let p1 = &samples[i];

            let p0_p = p - p0.pos;
            let p_p1 = p1.pos - p;

            let r1 = p0_p.dot(p0.der1);
            let r2 = p_p1.dot(p1.der1);

            let is_perpendicular =
                // perpendicular at p0 or p1
                (r1 == 0.0 || r2 == 0.0) ||
                // perpendicular from outside of curve or inside "focal point" 
                (allow_above_focal_point && r1 > 0.0 && r2 > 0.0) ||
                // perpendicular from below curve beyond the "focal point"
                (allow_below_focal_point && r1 < 0.0 && r2 < 0.0);

            if is_perpendicular {
                start_params.push((p0.u + p1.u) / 2.0);
            }
        }

        start_params
    }

    fn project_from_starting_param(
        &self,
        point: Point3,
        mut u: f64,
    ) -> Option<PointToCurveProjection> {
        const MAX_ITER: u32 = 32;

        let (u_min, u_max) = self.domain();

        for iter in 0..MAX_ITER {
            let cp = self.point(u);

            let pos = cp.pos();
            let d1 = cp.der1();
            let d2 = cp.der2();
            let diff = pos - point;
            let dist = diff.magnitude();
            let d1_dot_diff = d1.dot(diff);

            let delta_den = d2.dot(diff) + d1.magnitude2();

            let delta = if !delta_den.cc(0.0) {
                d1_dot_diff / delta_den
            } else {
                // The derivative (delta_den) of the function we're minimizing can
                // be zero in some situations, which would normally cause delta to
                // very large or infinite. This situation can often be fixed by
                // setting delta to some small fraction of the domain, "pushing"
                // the u parameter off the troublesome value.
                (u_max - u_min) / 100.0
            };

            let result = Some(PointToCurveProjection {
                iter,
                u,
                pos: *pos,
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
                if (d1_dot_diff / (d1.magnitude() * dist)).cc_newton(0.0) {
                    return result;
                }
            }

            // Get the next parameter value, making sure it stays in the domain
            let u_next = u - delta;
            let u_next = if self.is_closed() {
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

            // Additional stopping condition: parameter hasn't changed significantly or
            // is off the end of the curve (if unclosed).
            if ((u_next - u) * d1).magnitude().cc_newton(0.0) {
                return result;
            }

            u = u_next;
        }

        None
    }
}
impl From<LineSolver> for CurveSolver {
    fn from(line: LineSolver) -> Self {
        Self::new(CurveSolverKind::Line(line))
    }
}
impl From<ArcSolver> for CurveSolver {
    fn from(arc: ArcSolver) -> Self {
        Self::new(CurveSolverKind::Arc(arc))
    }
}
impl From<HelixSolver> for CurveSolver {
    fn from(helix: HelixSolver) -> Self {
        Self::new(CurveSolverKind::Helix(helix))
    }
}
