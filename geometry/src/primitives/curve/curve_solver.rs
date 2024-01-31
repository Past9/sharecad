use std::cell::OnceCell;

use crate::{
    math::{deg, vec2, Angle, Coincidence, Interval, Mat22, Quat, Scalar, Vec2, Vec3},
    primitives::Point,
    tessellate::{TessellatedCurve, TessellationTolerance},
};

use super::{ArcSolver, CurvePoint, HelixSolver, ICurvePoint, LineSolver};

pub trait ICurveSolver<S: Scalar> {
    type PointSolver: ICurvePoint<S>;

    fn domain(&self) -> (S, S);

    fn domain_span(&self) -> S {
        let (min, max) = self.domain();
        max - min
    }

    fn point(&self, u: S) -> Self::PointSolver;

    fn never_tangent(&self) -> &Vec3<S>;
}

#[derive(Clone, Debug)]
pub enum CurveSolverKind<S: Scalar> {
    Line(LineSolver<S>),
    Arc(ArcSolver<S>),
    Helix(HelixSolver<S>),
    //SSCurve(SSCurveSolver<S>),
}
impl CurveSolverKind<f64> {
    pub fn as_interval(&self) -> CurveSolverKind<Interval> {
        match self {
            CurveSolverKind::Line(line) => line.as_interval().into(),
            CurveSolverKind::Arc(arc) => arc.as_interval().into(),
            CurveSolverKind::Helix(helix) => helix.as_interval().into(),
        }
    }
}
impl<S: Scalar> From<LineSolver<S>> for CurveSolverKind<S> {
    fn from(solver: LineSolver<S>) -> Self {
        Self::Line(solver)
    }
}
impl<S: Scalar> From<ArcSolver<S>> for CurveSolverKind<S> {
    fn from(solver: ArcSolver<S>) -> Self {
        Self::Arc(solver)
    }
}
impl<S: Scalar> From<HelixSolver<S>> for CurveSolverKind<S> {
    fn from(solver: HelixSolver<S>) -> Self {
        Self::Helix(solver)
    }
}
/*
impl<S: Scalar> From<SSCurveSolver<S>> for CurveSolverKind<S> {
    fn from(solver: SSCurveSolver<S>) -> Self {
        Self::SSCurve(solver)
    }
}
 */

#[derive(Debug, Clone)]
pub struct PointToCurveProjection<S: Scalar> {
    pub iter: u32,
    pub u: S,
    pub pos: Vec3<S>,
    pub diff: Vec3<S>,
    pub dist: S,
}

#[derive(Debug, Clone)]
pub struct CCIntersection<S: Scalar> {
    pub u1: S,
    pub u2: S,
    pub pos: Vec3<S>,
}

#[derive(Debug)]
pub struct CurveSolver<S: Scalar> {
    kind: CurveSolverKind<S>,
    is_closed: OnceCell<bool>,
    //projection_tessellation: OnceCell<TessellatedCurve>,
}
impl<S: Scalar> CurveSolver<S> {
    pub(super) fn new(kind: CurveSolverKind<S>) -> Self {
        Self {
            kind,
            is_closed: OnceCell::new(),
            //projection_tessellation: OnceCell::new(),
        }
    }

    pub fn line(start: Point<S>, end: Point<S>) -> Self {
        LineSolver::new(start, end).into()
    }

    pub fn arc(r: S, angle: Angle<S>, orientation: Quat<S>, translation: Vec3<S>) -> Self {
        ArcSolver::new(r, angle, orientation, translation).into()
    }

    pub fn helix(r: S, h: S, n: S, orientation: Quat<S>, translation: Vec3<S>) -> Self {
        HelixSolver::new(r, h, n, orientation, translation).into()
    }

    pub fn domain(&self) -> (S, S) {
        match &self.kind {
            CurveSolverKind::Line(line) => line.domain(),
            CurveSolverKind::Helix(helix) => helix.domain(),
            CurveSolverKind::Arc(arc) => arc.domain(),
            //CurveSolverKind::SSCurve(ss_curve) => ss_curve.domain(),
        }
    }

    pub fn point(&self, u: S) -> CurvePoint<S> {
        match &self.kind {
            CurveSolverKind::Line(line) => CurvePoint::from(line.point(u)),
            CurveSolverKind::Helix(helix) => CurvePoint::from(helix.point(u)),
            CurveSolverKind::Arc(arc) => CurvePoint::from(arc.point(u)),
            //CurveSolverKind::SSCurve(ss_curve) => CurvePoint::from(ss_curve.point(u)),
        }
    }

    pub fn never_tangent(&self) -> &Vec3<S> {
        match &self.kind {
            CurveSolverKind::Line(line) => line.never_tangent(),
            CurveSolverKind::Helix(helix) => helix.never_tangent(),
            CurveSolverKind::Arc(arc) => arc.never_tangent(),
            //CurveSolverKind::SSCurve(ss_curve) => ss_curve.never_tangent(),
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

    /*
    pub fn invert_point(&self, point: Vec3<S>) -> Vec<PointToCurveProjection<S>> {
        let initial_guesses = self.projection_starting_params(point, true, false);
        let mut results = vec![];

        for guess in initial_guesses {
            if let Some(res) = self.project_from_starting_param(point, guess) {
                if res.dist.cc(S::ZERO) {
                    results.push(res);
                }
            }
        }

        results
    }

    pub fn project_point(&self, point: Vec3<S>) -> Vec<PointToCurveProjection<S>> {
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
        p: Vec3<S>,
        allow_above_focal_point: bool,
        allow_below_focal_point: bool,
    ) -> Vec<S> {
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
                (r1 == S::ZERO || r2 == S::ZERO) ||
                // perpendicular from outside of curve or inside "focal point"
                (allow_above_focal_point && r1 > S::ZERO && r2 > S::ZERO) ||
                // perpendicular from below curve beyond the "focal point"
                (allow_below_focal_point && r1 < S::ZERO && r2 < S::ZERO);

            if is_perpendicular {
                start_params.push((p0.u + p1.u) / S::TWO);
            }
        }

        start_params
    }

    fn project_from_starting_param(
        &self,
        point: Vec3<S>,
        mut u: S,
    ) -> Option<PointToCurveProjection<S>> {
        const MAX_ITER: u32 = 32;

        let (u_min, u_max) = self.domain();

        for iter in 0..MAX_ITER {
            let cp = self.point(u);

            let pos = cp.pos();
            let d1 = cp.der1();
            let d2 = cp.der2();
            let diff = *pos - point;
            let dist = diff.magnitude();
            let d1_dot_diff = d1.dot(diff);

            let delta_den = d2.dot(diff) + d1.magnitude2();

            let delta = if !delta_den.cc(S::ZERO) {
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
    */
}
impl CurveSolver<f64> {
    pub fn as_interval(&self) -> CurveSolver<Interval> {
        CurveSolver {
            kind: self.kind.as_interval(),
            is_closed: OnceCell::from(self.is_closed()),
        }
    }

    pub fn intersect_curve(&self, other: &Self) -> Vec<CCIntersection<Interval>> {
        let self_ivl: CurveSolver<Interval> = self.as_interval();
        let other_ivl: CurveSolver<Interval> = other.as_interval();

        let func = |u_ivls: Vec2<f64>| -> Vec2<Interval> {
            let p0 = self_ivl.point(Interval::thin(u_ivls.u()));
            let p1 = other_ivl.point(Interval::thin(u_ivls.v()));

            vec2(
                (*p0.pos() - *p1.pos()).dot(*p0.der1()),
                (*p1.pos() - *p0.pos()).dot(*p1.der1()),
            )
        };

        let jacobian = |u_ivls: Vec2<Interval>| -> Mat22<Interval> {
            let p0 = self_ivl.point(u_ivls.u());
            let p1 = other_ivl.point(u_ivls.v());

            let p0du = (*p0.pos() - *p1.pos()).dot(*p0.der2()) + p0.der1().magnitude2();
            let p0dv = (-*p1.der1()).dot(*p0.der1());

            let p1du = (-*p0.der1()).dot(*p1.der1());
            let p1dv = (*p1.pos() - *p0.pos()).dot(*p1.der2()) + p1.der1().magnitude2();

            Mat22::new(p0du, p0dv, p1du, p1dv)
        };

        let nf_der = |u_ivls: Vec2<Interval>, jacobian: Mat22<Interval>| -> Vec2<Interval> {
            /*
            let ji = match jacobian.inverse() {
                Some(ji) => ji,
                None => return vec2(Interval::EMPTY, Interval::EMPTY),
            };
             */
            println!("split eig = {:?}", jacobian.mid().eigenvalues());
            println!("is_reg = {}", jacobian.is_regular());

            let ji = jacobian.inverse().unwrap();
            //let ji = jacobian;
            //println!("ji * j = {:?}", (ji * jacobian).mid());
            //println!("JI = {:#?}", ji);
            let corrected = u_ivls.mid().as_interval() - ji * func(u_ivls.mid());
            println!("CORRECTED = {:#?} from {:#?}", corrected, u_ivls);
            (corrected).intersection(u_ivls)
        };

        /*
        let nf_der = |u_ivls: Vec2<Interval>, jacobian: Mat22<Interval>| -> Vec2<Interval> {
            let ji = jacobian.inverse().unwrap();
            println!("ji * j = {:?}", (ji * jacobian).mid());
            //println!("JI = {:#?}", ji);
            let corrected = u_ivls.mid().as_interval() - ji * func(u_ivls.mid());
            println!("CORRECTED = {:#?}", corrected);
            (corrected).intersection(u_ivls)
        };
         */

        const MAX_ITER: u32 = 50;
        let domains: Vec2<Interval> = vec2(self.domain().into(), other.domain().into());
        let mut search_intervals: Vec<Vec2<Interval>> = vec![domains];
        let mut converged_intervals: Vec<Vec2<Interval>> = vec![];

        #[derive(PartialEq, Debug, Copy, Clone)]
        struct NewInterval {
            new: Vec2<Interval>,
            from: Vec2<Interval>,
        }

        let mut iter = 0;
        while iter < MAX_ITER {
            iter += 1;

            println!("\niter = {}", iter);
            println!("SI len = {}", search_intervals.len());
            /*
            println!("search_intervals = {:#?}", search_intervals);
            println!("converged_intervals = {:#?}", converged_intervals);
             */

            if search_intervals.len() == 0 {
                break;
            }

            let mut new_intervals: Vec<NewInterval> = vec![];
            for search_interval in search_intervals {
                let mut pending_new_intervals = vec![];
                /*
                println!("jac = {:#?}", jacobian(search_interval));
                println!(
                    "num splits = {:#?}",
                    jacobian(search_interval).split_on_zero()
                );
                 */
                //for split in jacobian(search_interval).split_on_zero() {
                println!("JAC = {:#?}", jacobian(search_interval));
                println!(
                    "JAC eig = {:?}",
                    jacobian(search_interval).mid().eigenvalues()
                );
                println!("JAC is reg = {:?}", jacobian(search_interval).is_regular());
                for split in jacobian(search_interval).split_on_zero() {
                    pending_new_intervals.push(NewInterval {
                        new: nf_der(search_interval, split).intersection(domains),
                        from: search_interval,
                    });
                }

                println!("pending {:#?}", pending_new_intervals);

                for pending in pending_new_intervals.iter() {
                    if pending_new_intervals
                        .iter()
                        .filter(|p| p.new == pending.new)
                        .count()
                        == 1
                    {
                        new_intervals.push(*pending);
                    }
                }

                /*
                while pending_new_intervals.len() > 0 {
                    for pending in pending_new_intervals.iter() {
                        if pending_new_intervals
                            .iter()
                            .filter(|p| p.new == pending.new)
                            .count()
                            == 1
                        {
                            new_intervals.push(*pending);
                        }
                    }

                    /*
                    let last = pending_new_intervals.pop().unwrap();
                    if !pending_new_intervals
                        .iter()
                        .any(|remaining| remaining.new == last.new)
                    {
                        new_intervals.push(last);
                    } else {
                        //
                        //new_intervals.push(last);
                    }
                    */
                }
                */
            }

            search_intervals = vec![];

            for new_interval in new_intervals {
                println!("NI: {} from {}", new_interval.new, new_interval.from);
                if new_interval.new.is_empty() {
                    println!("EMPTY");
                    continue;
                }

                /*
                if new_interval.new.intersection(new_interval.from).is_empty() {
                    continue;
                }
                 */

                if new_interval.new.is_subset_of(new_interval.from) {
                    // If the new interval is a subset of the old one,
                    // then it contains exactly one zero
                    if new_interval.new == new_interval.from {
                        // If the new interval is equal to the old one,
                        // then it's done converging on the zero
                        converged_intervals.push(new_interval.new);
                        println!("CONVERGED");
                    } else {
                        // Otherwise, refine it again on the next iteration
                        search_intervals.push(new_interval.new);
                        println!("REFINE NEXT");
                    }
                } else {
                    // There are no zeros in the new interval, so discard it
                    println!("DISCARD");
                }
            }
        }

        println!("search_intervals = {:#?}", search_intervals);
        println!("converged_intervals = {:#?}", converged_intervals);

        vec![]
    }
}
impl<S: Scalar> From<LineSolver<S>> for CurveSolver<S> {
    fn from(line: LineSolver<S>) -> Self {
        Self::new(CurveSolverKind::Line(line))
    }
}
impl<S: Scalar> From<ArcSolver<S>> for CurveSolver<S> {
    fn from(arc: ArcSolver<S>) -> Self {
        Self::new(CurveSolverKind::Arc(arc))
    }
}
impl<S: Scalar> From<HelixSolver<S>> for CurveSolver<S> {
    fn from(helix: HelixSolver<S>) -> Self {
        Self::new(CurveSolverKind::Helix(helix))
    }
}
/*
impl<S: Scalar> From<SSCurveSolver<S>> for CurveSolver<S> {
    fn from(ss_curve: SSCurveSolver<S>) -> Self {
        Self::new(CurveSolverKind::SSCurve(ss_curve))
    }
}
*/
