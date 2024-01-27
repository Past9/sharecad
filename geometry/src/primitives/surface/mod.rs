mod surface_point;
mod surface_solver;
mod sweep;

use crate::{
    math::{deg, vec2, vec4, Coincidence, Mat33, Scalar, Vec2, Vec3, Vec4},
    PrimitiveGeometry,
};

pub use surface_point::*;
pub use surface_solver::*;
pub use sweep::*;

//use super::SSCurveParams;

#[derive(Debug)]
pub enum Surface<S: Scalar> {
    Sweep(Sweep<S>),
}
impl<S: Scalar> Surface<S> {
    pub fn solver(&self, geometry: &PrimitiveGeometry<S>) -> SurfaceSolver<S> {
        match self {
            Surface::Sweep(sweep) => SurfaceSolver::new(sweep.solver(geometry).into()),
        }
    }
}
impl<S: Scalar> From<Sweep<S>> for Surface<S> {
    fn from(sweep: Sweep<S>) -> Self {
        Self::Sweep(sweep)
    }
}

mod helpers {
    use crate::math::{Scalar, Vec2};

    use super::ISurfacePoint;

    pub fn ff1<S: Scalar, P: ISurfacePoint<S>>(point: &P) -> (S, S, S) {
        let (du, dv) = point.der1();

        let e = du.dot(*du);
        let f = du.dot(*dv);
        let g = dv.dot(*dv);

        (e, f, g)
    }

    pub fn ff2<S: Scalar, P: ISurfacePoint<S>>(point: &P) -> (S, S, S) {
        let (du, dv) = point.der1();
        let (duu, duv, dvv) = point.der2();

        let normal = dv.cross(*du).normalize();

        let l = duu.dot(normal);
        let m = duv.dot(normal);
        let n = dvv.dot(normal);

        (l, m, n)
    }

    pub fn normal_curvature<S: Scalar, P: ISurfacePoint<S>>(point: &P, direction: Vec2<S>) -> S {
        let (e, f, g) = *point.ff1();
        let (l, m, n) = *point.ff2();

        let du2 = direction.u().powi(2);
        let dudv = direction.u() * direction.v();
        let dv2 = direction.v().powi(2);

        (l * du2 + S::TWO * m * dudv + n * dv2) / (e * du2 + S::TWO * f * dudv + g * dv2)
    }

    pub fn mean_curvature<S: Scalar, P: ISurfacePoint<S>>(point: &P) -> S {
        let (e, f, g) = *point.ff1();
        let (l, m, n) = *point.ff2();

        S::HALF * (e * n - S::TWO * f * m + g * l) / (e * g - f.powi(2))
    }

    pub fn gaussian_curvature<S: Scalar, P: ISurfacePoint<S>>(point: &P) -> S {
        let (e, f, g) = *point.ff1();
        let (l, m, n) = *point.ff2();
        (l * n - m.powi(2)) / (e * g - f.powi(2))
    }

    pub fn principal_curvatures<S: Scalar, P: ISurfacePoint<S>>(point: &P) -> (S, S) {
        let h = point.mean_curvature();
        let k = point.gaussian_curvature();

        let root = (h.powi(2) - k).sqrt();

        (h + root, h - root)
    }
}

pub struct SITResult {}

/*
pub struct SSCurveSampler<'a, S: Scalar> {
    points: Vec<SSCurveParams<S>>,
    s0: &'a SurfaceSolver<S>,
    s1: &'a SurfaceSolver<S>,
}
impl<'a, S: Scalar> SSCurveSampler<'a, S> {
    pub fn new(
        s0: &'a SurfaceSolver<S>,
        s1: &'a SurfaceSolver<S>,
        uv0: Vec2<S>,
        uv1: Vec2<S>,
    ) -> Self {
        Self::new_from_starting_params(
            s0,
            s1,
            SSCurveParams {
                u: S::ZERO,
                pos: Self::curve_pos(s0, s1, uv0, uv1),
                s0: uv0,
                s1: uv1,
            },
        )
    }

    pub fn start(&self) -> &SSCurveParams<S> {
        &self.points[0]
    }

    pub fn last(&self) -> &SSCurveParams<S> {
        &self.points[self.points.len() - 1]
    }

    pub fn take_points(self) -> Vec<SSCurveParams<S>> {
        self.points
    }

    pub fn new_from_starting_params(
        s0: &'a SurfaceSolver<S>,
        s1: &'a SurfaceSolver<S>,
        start_params: SSCurveParams<S>,
    ) -> Self {
        Self {
            points: vec![start_params],
            s0,
            s1,
        }
    }

    fn curve_pos(
        s0: &SurfaceSolver<S>,
        s1: &SurfaceSolver<S>,
        uv0: Vec2<S>,
        uv1: Vec2<S>,
    ) -> Vec3<S> {
        let s0_point = *s0.point(uv0).pos();
        let s1_point = *s1.point(uv1).pos();
        (s0_point + s1_point) / S::TWO
    }

    pub fn fill(&mut self, max_step: S) {
        while let Some(next) = self.next(max_step) {
            self.points.push(next);
        }
    }

    pub fn next(&self, max_step: S) -> Option<SSCurveParams<S>> {
        let start = self.start();
        let previous = self.last();
        if self.points.len() > 1 && previous.pos.cc(start.pos) {
            return None;
        }

        let next = Self::rk_step(&self.s0, &self.s1, previous, max_step);

        let to_next = next.pos - previous.pos;
        let to_start = start.pos - previous.pos;

        let dist_to_start = to_start.magnitude();

        if to_next.dot(to_start) > S::ZERO && dist_to_start < max_step {
            self.next(dist_to_start)
        } else {
            Some(next)
        }
    }

    pub fn rk_step(
        s0: &'a SurfaceSolver<S>,
        s1: &'a SurfaceSolver<S>,
        // Current surface UVs as curve param
        from: &SSCurveParams<S>,
        // Step size along u (curve param)
        h: S,
    ) -> SSCurveParams<S> {
        let y = vec4(from.s0.u(), from.s0.v(), from.s1.u(), from.s1.v());

        let k1 = h * Self::ders(s0, s1, y);
        let k2 = h * Self::ders(s0, s1, y + S::HALF * k1);
        let k3 = h * Self::ders(s0, s1, y + S::HALF * k2);
        let k4 = h * Self::ders(s0, s1, y + k3);

        let next = y + (S::ONE / S::exact(6.0)) * (k1 + S::TWO * k2 + S::TWO * k3 + k4);

        let next_uv0 = vec2(next.x, next.y);
        let next_uv1 = vec2(next.z, next.w);

        let next_pos = Self::curve_pos(s0, s1, next_uv0, next_uv1);

        SSCurveParams {
            u: from.u + h,
            pos: next_pos,
            s0: next_uv0,
            s1: next_uv1,
        }
    }

    fn ders(s0: &'a SurfaceSolver<S>, s1: &'a SurfaceSolver<S>, uvs: Vec4<S>) -> Vec4<S> {
        let s0_point = s0.point(vec2(uvs.x, uvs.y));
        let s1_point = s1.point(vec2(uvs.z, uvs.w));

        let (s0_du, s0_dv) = *s0_point.der1();
        let s0_normal = s0_du.cross(s0_dv);

        let (s1_du, s1_dv) = *s1_point.der1();
        let s1_normal = s1_du.cross(s1_dv);

        let c = s0_normal.cross(s1_normal).normalize();

        let d_u0 = Mat33::from_col_vecs(c, s0_dv, s0_normal).determinant() / s0_normal.magnitude2();
        let d_v0 = Mat33::from_col_vecs(s0_du, c, s0_normal).determinant() / s0_normal.magnitude2();
        let d_u1 = Mat33::from_col_vecs(c, s1_dv, s1_normal).determinant() / s1_normal.magnitude2();
        let d_v1 = Mat33::from_col_vecs(s1_du, c, s1_normal).determinant() / s1_normal.magnitude2();

        vec4(d_u0, d_v0, d_u1, d_v1)
    }
}
 */

pub struct SurfaceIntersection<'a, S: Scalar> {
    s0: &'a SurfaceSolver<S>,
    s1: &'a SurfaceSolver<S>,
}
impl<'a, S: Scalar> SurfaceIntersection<'a, S> {
    pub fn new(s0: &'a SurfaceSolver<S>, s1: &'a SurfaceSolver<S>) -> Self {
        Self { s0, s1 }
    }

    pub fn next(&self, uv0: Vec2<S>, uv1: Vec2<S>) -> (Vec2<S>, Vec2<S>) {
        println!();
        let params = vec4(uv0.u(), uv0.v(), uv1.u(), uv1.v());
        let gradient = self.gradient(uv0, uv1);

        let vlen = deg(S::exact(360.0)).radians();
        let ulen = S::ONE;

        let gradient = vec4(
            gradient.x / ulen,
            gradient.y / vlen,
            gradient.z / ulen,
            gradient.w / vlen,
        );

        println!("uv0, uv1 = {}, {}", uv0, uv1);
        println!("gradient {}", gradient);
        println!("gradient.magnitude() {}", gradient.magnitude());
        println!("dist2 {}", self.dist2(uv0, uv1));
        println!("dist {}", self.dist2(uv0, uv1).sqrt());
        //let sub = 1.0 * self.dist2(uv0, uv1) / self.gradient(uv0, uv1);
        let sub = S::exact(0.001) * gradient.normalize();
        println!("sub {}", sub);
        println!("sub.magnitude() {}", sub.magnitude());
        let Vec4 { x, y, z, w } = params - (sub);
        let out = (vec2(x, y), vec2(z, w));

        println!("out {:?}", out);

        out
    }

    pub fn dist2(&self, uv0: Vec2<S>, uv1: Vec2<S>) -> S {
        (*self.s0.point(uv0).pos() - *self.s1.point(uv1).pos()).magnitude2()
    }

    pub fn gradient(&self, uv0: Vec2<S>, uv1: Vec2<S>) -> Vec4<S> {
        let s0_point = self.s0.point(uv0);
        let s0_pos = *s0_point.pos();
        let (s0_du, s0_dv) = *s0_point.der1();

        let s1_point = self.s1.point(uv1);
        let s1_pos = *s1_point.pos();
        let (s1_du, s1_dv) = *s1_point.der1();

        let d_u0 = (s0_pos - s1_pos).dot(s0_du) + s0_du.dot(s0_pos - s1_pos);
        let d_v0 = (s0_pos - s1_pos).dot(s0_dv) + s0_dv.dot(s0_pos - s1_pos);
        let d_u1 = (s0_pos - s1_pos).dot(-s1_du) + (-s1_du).dot(s0_pos - s1_pos);
        let d_v1 = (s0_pos - s1_pos).dot(-s1_dv) + (-s1_dv).dot(s0_pos - s1_pos);

        vec4(d_u0, d_v0, d_u1, d_v1)
        //vec4(d_v1, d_u1, d_v0, d_u0)
        //vec4(-d_v0, d_u0, d_v1, -d_u1)
    }

    /*
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
    */
}
