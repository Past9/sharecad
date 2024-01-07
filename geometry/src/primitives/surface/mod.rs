mod surface_point;
mod surface_solver;
mod sweep;

use crate::{
    math::{deg, point2, vec4, Mat33, Mat44, Point2, Vec4},
    PrimitiveGeometry,
};

pub use surface_point::*;
pub use surface_solver::*;
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

pub struct SITResult {}

pub struct SurfaceIntersectionTransversal<'a> {
    s0: &'a SurfaceSolver,
    s1: &'a SurfaceSolver,
}
impl<'a> SurfaceIntersectionTransversal<'a> {
    pub fn new(s0: &'a SurfaceSolver, s1: &'a SurfaceSolver) -> Self {
        Self { s0, s1 }
    }

    pub fn at(&self, uv0: Point2, uv1: Point2) -> SITResult {
        let s0_point = self.s0.point(uv0);
        let s1_point = self.s1.point(uv1);

        let (s0_du, s0_dv) = *s0_point.der1();
        let s0_normal = s0_du.cross(s0_dv).normalize();

        let (s1_du, s1_dv) = *s1_point.der1();
        let s1_normal = s1_du.cross(s1_dv).normalize();

        let c = s0_normal.cross(s1_normal).normalize();

        let d_u0 = Mat33::from_col_vecs(c, s0_dv, s0_normal).determinant() / s0_normal.magnitude2();
        let d_v0 = Mat33::from_col_vecs(s0_du, c, s0_normal).determinant() / s0_normal.magnitude2();
        let d_u1 = Mat33::from_col_vecs(c, s1_dv, s1_normal).determinant() / s1_normal.magnitude2();
        let d_v1 = Mat33::from_col_vecs(s1_du, c, s1_normal).determinant() / s1_normal.magnitude2();

        SITResult {}
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
        println!();
        let params = vec4(uv0.u(), uv0.v(), uv1.u(), uv1.v());
        let gradient = self.gradient(uv0, uv1);

        let vlen = deg(360.0).radians();
        let ulen = 1.0;

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
        let sub = 0.001 * gradient.normalize();
        println!("sub {}", sub);
        println!("sub.magnitude() {}", sub.magnitude());
        let Vec4 { x, y, z, w } = params - (sub);
        let out = (point2(x, y), point2(z, w));

        println!("out {:?}", out);

        out
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
