use crate::{
    math::{point2, Mat33, Point2, Point3, Vec2, Vec3},
    primitives::{axes, axes_der1, axes_der2, CurvePoint, CurveSolver},
    IGeometry, PrimitiveGeometry,
};
use common::CurveId;
use std::cell::OnceCell;

use super::{
    helpers::{
        ff1, ff2, gaussian_curvature, mean_curvature, normal_curvature, principal_curvatures,
    },
    ISurfacePoint, ISurfaceSolver,
};

#[derive(Clone, Debug)]
pub struct Sweep {
    profile: CurveId,
    path: CurveId,
}
impl Sweep {
    pub fn new(profile: CurveId, path: CurveId) -> Self {
        Self { profile, path }
    }

    pub fn solver(&self, geometry: &PrimitiveGeometry) -> SweepSolver {
        SweepSolver {
            profile: geometry
                .curve(self.profile)
                .unwrap()
                .to_owned()
                .solver(geometry),
            path: geometry
                .curve(self.path)
                .unwrap()
                .to_owned()
                .solver(geometry),
        }
    }
}

pub struct SweepSolver {
    profile: CurveSolver,
    path: CurveSolver,
}
impl SweepSolver {
    pub fn new(profile: CurveSolver, path: CurveSolver) -> Self {
        Self { profile, path }
    }
}
impl<'a> ISurfaceSolver<'a> for SweepSolver {
    type Point = SweepPoint<'a>;

    fn domain(&self) -> (Point2, Point2) {
        let (u_min, u_max) = self.profile.domain();
        let (v_min, v_max) = self.path.domain();
        (point2(u_min, v_min), point2(u_max, v_max))
    }

    fn point(&'a self, uv: Point2) -> Self::Point {
        SweepPoint::new(&self, uv)
    }
}

pub struct SweepPoint<'a> {
    profile_u: CurvePoint<'a>,
    path_v: CurvePoint<'a>,
    path_start: CurvePoint<'a>,

    path_axes_start_inverse_mat: OnceCell<Mat33>,

    path_axes: OnceCell<(Vec3, Vec3, Vec3)>,
    path_axes_mat: OnceCell<Mat33>,

    path_axes_der1: OnceCell<(Vec3, Vec3, Vec3)>,
    path_axes_der1_mat: OnceCell<Mat33>,

    path_axes_der2: OnceCell<(Vec3, Vec3, Vec3)>,
    path_axes_der2_mat: OnceCell<Mat33>,

    sweep: &'a SweepSolver,
    uv: Point2,
    eval: OnceCell<Point3>,
    der1: OnceCell<(Vec3, Vec3)>,
    der2: OnceCell<(Vec3, Vec3, Vec3)>,
    ff1: OnceCell<(f64, f64, f64)>,
    ff2: OnceCell<(f64, f64, f64)>,
    normal_curvature: OnceCell<f64>,
    mean_curvature: OnceCell<f64>,
    gaussian_curvature: OnceCell<f64>,
    principal_curvatures: OnceCell<(f64, f64)>,
}
impl<'a> SweepPoint<'a> {
    pub fn new(sweep: &'a SweepSolver, uv: Point2) -> Self {
        Self {
            profile_u: sweep.profile.point(uv.u()),
            path_v: sweep.path.point(uv.v()),

            path_start: sweep.path.point(sweep.domain().0.y),
            path_axes_start_inverse_mat: OnceCell::new(),

            path_axes: OnceCell::new(),
            path_axes_mat: OnceCell::new(),
            path_axes_der1: OnceCell::new(),
            path_axes_der1_mat: OnceCell::new(),
            path_axes_der2: OnceCell::new(),
            path_axes_der2_mat: OnceCell::new(),

            sweep,
            uv,
            //path_axes: OnceCell::new(),
            eval: OnceCell::new(),
            der1: OnceCell::new(),
            der2: OnceCell::new(),
            ff1: OnceCell::new(),
            ff2: OnceCell::new(),
            normal_curvature: OnceCell::new(),
            mean_curvature: OnceCell::new(),
            gaussian_curvature: OnceCell::new(),
            principal_curvatures: OnceCell::new(),
        }
    }

    pub fn path_axes(&self) -> &(Vec3, Vec3, Vec3) {
        self.path_axes
            .get_or_init(|| axes(&self.path_v, &self.sweep.path.never_tangent()))
    }

    pub fn path_axes_mat(&self) -> &Mat33 {
        self.path_axes_mat.get_or_init(|| {
            let (x, y, z) = *self.path_axes();
            Mat33::from_axes(x, y, z)
        })
    }

    pub fn path_axes_der1(&self) -> &(Vec3, Vec3, Vec3) {
        self.path_axes_der1.get_or_init(|| {
            axes_der1(
                &self.path_v,
                &self.sweep.path.never_tangent(),
                self.path_axes(),
            )
        })
    }

    pub fn path_axes_der1_mat(&self) -> &Mat33 {
        self.path_axes_der1_mat.get_or_init(|| {
            let (x, y, z) = *self.path_axes_der1();
            Mat33::from_axes(x, y, z)
        })
    }

    pub fn path_axes_der2(&self) -> &(Vec3, Vec3, Vec3) {
        self.path_axes_der2.get_or_init(|| {
            axes_der2(
                &self.path_v,
                &self.sweep.path.never_tangent(),
                self.path_axes(),
                self.path_axes_der1(),
            )
        })
    }

    pub fn path_axes_der2_mat(&self) -> &Mat33 {
        self.path_axes_der2_mat.get_or_init(|| {
            let (x, y, z) = *self.path_axes_der2();
            Mat33::from_axes(x, y, z)
        })
    }

    pub fn path_axes_start_inverse_mat(&self) -> &Mat33 {
        self.path_axes_start_inverse_mat.get_or_init(|| {
            let (x, y, z) = axes(&self.path_start, self.sweep.path.never_tangent());
            let matrix = Mat33::from_axes(x, y, z);
            matrix.inverse().unwrap()
        })
    }
}
impl<'a> ISurfacePoint for SweepPoint<'a> {
    fn uv(&self) -> Point2 {
        self.uv
    }

    fn pos(&self) -> &Point3 {
        self.eval.get_or_init(|| {
            let m = self.path_axes_mat() * self.path_axes_start_inverse_mat();
            self.path_v.pos() + m * (self.profile_u.pos() - self.path_start.pos())
        })
    }

    fn der1(&self) -> &(Vec3, Vec3) {
        self.der1.get_or_init(|| {
            let m = self.path_axes_mat() * self.path_axes_start_inverse_mat();
            let du = m * self.profile_u.der1();

            let m_der1 = self.path_axes_der1_mat() * self.path_axes_start_inverse_mat();
            let dv = self.path_v.der1() + m_der1 * (self.profile_u.pos() - self.path_start.pos());

            (du, dv)
        })
    }

    fn der2(&self) -> &(Vec3, Vec3, Vec3) {
        self.der2.get_or_init(|| {
            let m = self.path_axes_mat() * self.path_axes_start_inverse_mat();
            let duu = m * self.profile_u.der2();

            let m_der1 = self.path_axes_der1_mat() * self.path_axes_start_inverse_mat();
            let duv = m_der1 * self.profile_u.der1();

            let m_der2 = self.path_axes_der2_mat() * self.path_axes_start_inverse_mat();
            let dvv = self.path_v.der2() + m_der2 * (self.profile_u.pos() - self.path_start.pos());

            (duu, duv, dvv)
        })
    }

    fn ff1(&self) -> &(f64, f64, f64) {
        self.ff1.get_or_init(|| ff1(self))
    }

    fn ff2(&self) -> &(f64, f64, f64) {
        self.ff2.get_or_init(|| ff2(self))
    }

    fn normal_curvature(&self, direction: Vec2) -> f64 {
        *self
            .normal_curvature
            .get_or_init(|| normal_curvature(self, direction))
    }

    fn mean_curvature(&self) -> f64 {
        *self.mean_curvature.get_or_init(|| mean_curvature(self))
    }

    fn gaussian_curvature(&self) -> f64 {
        *self
            .gaussian_curvature
            .get_or_init(|| gaussian_curvature(self))
    }

    fn principal_curvatures(&self) -> &(f64, f64) {
        self.principal_curvatures
            .get_or_init(|| principal_curvatures(self))
    }
}
