mod sweep;

use std::{cell::OnceCell, time::Instant};

use crate::{
    math::{point2, vec2, vec4, Coincidence, Mat22, Mat44, Point2, Point3, Vec2, Vec3, Vec4},
    primitives::curve::CurveSolver,
    PrimitiveGeometry,
};

pub use sweep::*;

#[derive(Debug)]
pub enum Surface {
    Sweep(Sweep),
}
impl Surface {
    pub fn solver(&self, geometry: &PrimitiveGeometry) -> SurfaceSolver {
        match self {
            Surface::Sweep(sweep) => SurfaceSolver::new(sweep.solver(geometry).into()),
        }
    }
}
impl From<Sweep> for Surface {
    fn from(sweep: Sweep) -> Self {
        Self::Sweep(sweep)
    }
}

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

pub trait ISurfacePoint {
    fn u(&self) -> f64 {
        self.uv().u()
    }

    fn v(&self) -> f64 {
        self.uv().v()
    }

    fn uv(&self) -> Point2;
    fn pos(&self) -> &Point3;
    fn der1(&self) -> &(Vec3, Vec3);
    fn der2(&self) -> &(Vec3, Vec3, Vec3);
    fn ff1(&self) -> &(f64, f64, f64);
    fn ff2(&self) -> &(f64, f64, f64);
    fn normal_curvature(&self, direction: Vec2) -> f64;
    fn mean_curvature(&self) -> f64;
    fn gaussian_curvature(&self) -> f64;
    fn principal_curvatures(&self) -> &(f64, f64);
}

pub enum SurfacePoint<'a> {
    Sweep(SweepPoint<'a>),
}
impl<'a> SurfacePoint<'a> {
    pub fn u(&self) -> f64 {
        match self {
            SurfacePoint::Sweep(sweep) => sweep.u(),
        }
    }

    pub fn v(&self) -> f64 {
        match self {
            SurfacePoint::Sweep(sweep) => sweep.v(),
        }
    }

    pub fn uv(&self) -> Point2 {
        match self {
            SurfacePoint::Sweep(sweep) => sweep.uv(),
        }
    }

    pub fn pos(&self) -> &Point3 {
        match self {
            SurfacePoint::Sweep(sweep) => sweep.pos(),
        }
    }

    pub fn der1(&self) -> &(Vec3, Vec3) {
        match self {
            SurfacePoint::Sweep(sweep) => sweep.der1(),
        }
    }

    pub fn der2(&self) -> &(Vec3, Vec3, Vec3) {
        match self {
            SurfacePoint::Sweep(sweep) => sweep.der2(),
        }
    }

    pub fn ff1(&self) -> &(f64, f64, f64) {
        match self {
            SurfacePoint::Sweep(sweep) => sweep.ff1(),
        }
    }

    pub fn ff2(&self) -> &(f64, f64, f64) {
        match self {
            SurfacePoint::Sweep(sweep) => sweep.ff2(),
        }
    }

    pub fn normal_curvature(&self, direction: Vec2) -> f64 {
        match self {
            SurfacePoint::Sweep(sweep) => sweep.normal_curvature(direction),
        }
    }

    pub fn mean_curvature(&self) -> f64 {
        match self {
            SurfacePoint::Sweep(sweep) => sweep.mean_curvature(),
        }
    }

    pub fn gaussian_curvature(&self) -> f64 {
        match self {
            SurfacePoint::Sweep(sweep) => sweep.gaussian_curvature(),
        }
    }

    pub fn principal_curvatures(&self) -> &(f64, f64) {
        match self {
            SurfacePoint::Sweep(sweep) => sweep.principal_curvatures(),
        }
    }
}
impl<'a> From<SweepPoint<'a>> for SurfacePoint<'a> {
    fn from(point: SweepPoint<'a>) -> Self {
        Self::Sweep(point)
    }
}

mod helpers {
    use crate::math::Vec2;

    use super::ISurfacePoint;

    pub fn ff1<P: ISurfacePoint>(point: &P) -> (f64, f64, f64) {
        let (du, dv) = point.der1();

        let e = du.dot(*du);
        let f = du.dot(*dv);
        let g = dv.dot(*dv);

        (e, f, g)
    }

    pub fn ff2<P: ISurfacePoint>(point: &P) -> (f64, f64, f64) {
        let (du, dv) = point.der1();
        let (duu, duv, dvv) = point.der2();

        let normal = dv.cross(*du).normalize();

        let l = duu.dot(normal);
        let m = duv.dot(normal);
        let n = dvv.dot(normal);

        (l, m, n)
    }

    pub fn normal_curvature<P: ISurfacePoint>(point: &P, direction: Vec2) -> f64 {
        let (e, f, g) = point.ff1();
        let (l, m, n) = point.ff2();

        let du2 = direction.u().powi(2);
        let dudv = direction.u() * direction.v();
        let dv2 = direction.v().powi(2);

        (l * du2 + 2.0 * m * dudv + n * dv2) / (e * du2 + 2.0 * f * dudv + g * dv2)
    }

    pub fn mean_curvature<P: ISurfacePoint>(point: &P) -> f64 {
        let (e, f, g) = point.ff1();
        let (l, m, n) = point.ff2();

        0.5 * (e * n - 2.0 * f * m + g * l) / (e * g - f.powi(2))
    }

    pub fn gaussian_curvature<P: ISurfacePoint>(point: &P) -> f64 {
        let (e, f, g) = point.ff1();
        let (l, m, n) = point.ff2();
        (l * n - m.powi(2)) / (e * g - f.powi(2))
    }

    pub fn principal_curvatures<P: ISurfacePoint>(point: &P) -> (f64, f64) {
        let h = point.mean_curvature();
        let k = point.gaussian_curvature();

        let root = (h.powi(2) - k).sqrt();

        (h + root, h - root)
    }
}

pub struct SurfaceIntersection<'a> {
    s0: &'a SurfaceSolver,
    s1: &'a SurfaceSolver,
}
impl<'a> SurfaceIntersection<'a> {
    pub fn new(s0: &'a SurfaceSolver, s1: &'a SurfaceSolver) -> Self {
        Self { s0, s1 }
    }

    pub fn next(&self, uv0: Point2, uv1: Point2) -> (Point2, Point2) {
        let params = vec4(uv0.u(), uv0.v(), uv1.u(), uv1.v());
        let sub = self.dist2(uv0, uv1) / self.gradient(uv0, uv1);
        let Vec4 { x, y, z, w } = params - sub;
        (point2(x, y), point2(z, w))
    }

    pub fn dist2(&self, uv0: Point2, uv1: Point2) -> f64 {
        (self.s0.point(uv0).pos() - self.s1.point(uv1).pos()).magnitude2()
    }

    pub fn gradient(&self, uv0: Point2, uv1: Point2) -> Vec4 {
        let s0_point = self.s0.point(uv0);
        let s0_pos = s0_point.pos();
        let (s0_du, s0_dv) = *s0_point.der1();

        let s1_point = self.s1.point(uv1);
        let s1_pos = s1_point.pos();
        let (s1_du, s1_dv) = *s1_point.der1();

        let d_u0 = (s0_pos - s1_pos).dot(s0_du) + s0_du.dot(s0_pos - s1_pos);
        let d_v0 = (s0_pos - s1_pos).dot(s0_dv) + s0_dv.dot(s0_pos - s1_pos);
        let d_u1 = (s0_pos - s1_pos).dot(-s1_du) + (-s1_du).dot(s0_pos - s1_pos);
        let d_v1 = (s0_pos - s1_pos).dot(-s1_dv) + (-s1_dv).dot(s0_pos - s1_pos);

        vec4(d_u0, d_v0, d_u1, d_v1)
    }

    pub fn hessian(&self, uv0: Point2, uv1: Point2) -> Mat44 {
        let s0_point = self.s0.point(uv0);
        let s0_pos = *s0_point.pos();
        let (s0_du, s0_dv) = *s0_point.der1();
        let (s0_duu, s0_duv, s0_dvv) = *s0_point.der2();

        let s1_point = self.s1.point(uv1);
        let s1_pos = *s1_point.pos();
        let (s1_du, s1_dv) = *s1_point.der1();
        let (s1_duu, s1_duv, s1_dvv) = *s1_point.der2();

        // Row 0
        let aa =
            (s0_pos - s1_pos).dot(s0_duu) + 2.0 * s0_du.dot(s0_du) + s0_duu.dot(s0_pos - s1_pos);

        let ab = (s0_pos - s1_pos).dot(s0_duv)
            + s0_dv.dot(s0_du)
            + s0_du.dot(s0_dv)
            + s0_duv.dot(s0_pos - s1_pos);

        let ac = s0_du.dot(-s1_du) + (-s1_du).dot(s0_du);

        let ad = (-s1_dv).dot(s0_du) + s0_du.dot(-s1_dv);

        // Row 1
        let ba = (s0_pos - s1_pos).dot(s0_duv)
            + s0_dv.dot(s0_du)
            + s0_du.dot(s0_dv)
            + s0_duv.dot(s0_pos - s1_pos);

        let bb =
            (s0_pos - s1_pos).dot(s0_dvv) + 2.0 * s0_dv.dot(s0_dv) + s0_dvv.dot(s0_pos - s1_pos);

        let bc = s0_dv.dot(-s1_du) + (-s1_du).dot(s0_dv);

        let bd = s0_dv.dot(-s1_dv) + (-s1_dv).dot(s0_dv);

        // Row 2
        let ca = s0_du.dot(-s1_du) + (-s1_du).dot(s0_du);

        let cb = s0_dv.dot(-s1_du) + (-s1_du).dot(s0_dv);

        let cc = (s0_pos - s1_pos).dot(-s1_duu)
            + 2.0 * (-s1_du).dot(-s1_du)
            + (-s1_duu).dot(s0_pos - s1_pos);

        let cd = (s0_pos - s1_pos).dot(-s1_duv)
            + (-s1_dv).dot(-s1_du)
            + (-s1_du).dot(-s1_dv)
            + (-s1_duv).dot(s0_pos - s1_pos);

        // Row 3
        let da = (-s1_dv).dot(s0_du) + s0_du.dot(-s1_dv);

        let db = s0_dv.dot(-s1_dv) + (-s1_dv).dot(s0_dv);

        let dc = (s0_pos - s1_pos).dot(-s1_duv)
            + (-s1_dv).dot(-s1_du)
            + (-s1_du).dot(-s1_dv)
            + (-s1_duv).dot(s0_pos - s1_pos);

        let dd = (s0_pos - s1_pos).dot(-s1_dvv)
            + 2.0 * (-s1_dv).dot(-s1_dv)
            + (-s1_dvv).dot(s0_pos - s1_pos);

        Mat44::new(
            aa, ab, ac, ad, //
            ba, bb, bc, bd, //
            ca, cb, cc, cd, //
            da, db, dc, dd, //
        )
    }
}
