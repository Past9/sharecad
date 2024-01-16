use crate::{
    math::{vec2, Mat33, Scalar, Vec2, Vec3},
    primitives::{axes, axes_der1, axes_der2, CurvePoint, CurveSolver},
    IGeometry, PrimitiveGeometry,
};
use common::CurveId;
use std::{cell::OnceCell, marker::PhantomData};

use super::{
    helpers::{
        ff1, ff2, gaussian_curvature, mean_curvature, normal_curvature, principal_curvatures,
    },
    ISurfacePoint, ISurfaceSolver,
};

#[derive(Clone, Debug)]
pub struct Sweep<S: Scalar> {
    profile: CurveId,
    path: CurveId,
    _s: PhantomData<S>,
}
impl<S: Scalar> Sweep<S> {
    pub fn new(profile: CurveId, path: CurveId) -> Self {
        Self {
            profile,
            path,
            _s: PhantomData,
        }
    }

    pub fn solver(&self, geometry: &PrimitiveGeometry<S>) -> SweepSolver<S> {
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

#[derive(Debug)]
pub struct SweepSolver<S: Scalar> {
    profile: CurveSolver<S>,
    path: CurveSolver<S>,
}
impl<S: Scalar> SweepSolver<S> {
    pub fn new(profile: CurveSolver<S>, path: CurveSolver<S>) -> Self {
        Self { profile, path }
    }
}
impl<'a, S: Scalar + 'a> ISurfaceSolver<'a, S> for SweepSolver<S> {
    type Point = SweepPoint<'a, S>;

    fn domain(&self) -> (Vec2<S>, Vec2<S>) {
        let (u_min, u_max) = self.profile.domain();
        let (v_min, v_max) = self.path.domain();
        (vec2(u_min, v_min), vec2(u_max, v_max))
    }

    fn point(&'a self, uv: Vec2<S>) -> Self::Point {
        SweepPoint::new(&self, uv)
    }
}

pub struct SweepPoint<'a, S: Scalar> {
    profile_u: CurvePoint<S>,
    path_v: CurvePoint<S>,
    path_start: CurvePoint<S>,

    path_axes_start_inverse_mat: OnceCell<Mat33<S>>,

    path_axes: OnceCell<(Vec3<S>, Vec3<S>, Vec3<S>)>,
    path_axes_mat: OnceCell<Mat33<S>>,

    path_axes_der1: OnceCell<(Vec3<S>, Vec3<S>, Vec3<S>)>,
    path_axes_der1_mat: OnceCell<Mat33<S>>,

    path_axes_der2: OnceCell<(Vec3<S>, Vec3<S>, Vec3<S>)>,
    path_axes_der2_mat: OnceCell<Mat33<S>>,

    sweep: &'a SweepSolver<S>,
    uv: Vec2<S>,
    eval: OnceCell<Vec3<S>>,
    der1: OnceCell<(Vec3<S>, Vec3<S>)>,
    der2: OnceCell<(Vec3<S>, Vec3<S>, Vec3<S>)>,
    ff1: OnceCell<(S, S, S)>,
    ff2: OnceCell<(S, S, S)>,
    normal_curvature: OnceCell<S>,
    mean_curvature: OnceCell<S>,
    gaussian_curvature: OnceCell<S>,
    principal_curvatures: OnceCell<(S, S)>,
}
impl<'a, S: Scalar> SweepPoint<'a, S> {
    pub fn new(sweep: &'a SweepSolver<S>, uv: Vec2<S>) -> Self {
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

    pub fn path_axes(&self) -> &(Vec3<S>, Vec3<S>, Vec3<S>) {
        self.path_axes
            .get_or_init(|| axes(&self.path_v, &self.sweep.path.never_tangent()))
    }

    pub fn path_axes_mat(&self) -> &Mat33<S> {
        self.path_axes_mat.get_or_init(|| {
            let (x, y, z) = *self.path_axes();
            Mat33::from_col_vecs(x, y, z)
        })
    }

    pub fn path_axes_der1(&self) -> &(Vec3<S>, Vec3<S>, Vec3<S>) {
        self.path_axes_der1.get_or_init(|| {
            axes_der1(
                &self.path_v,
                &self.sweep.path.never_tangent(),
                self.path_axes(),
            )
        })
    }

    pub fn path_axes_der1_mat(&self) -> &Mat33<S> {
        self.path_axes_der1_mat.get_or_init(|| {
            let (x, y, z) = *self.path_axes_der1();
            Mat33::from_col_vecs(x, y, z)
        })
    }

    pub fn path_axes_der2(&self) -> &(Vec3<S>, Vec3<S>, Vec3<S>) {
        self.path_axes_der2.get_or_init(|| {
            axes_der2(
                &self.path_v,
                &self.sweep.path.never_tangent(),
                self.path_axes(),
                self.path_axes_der1(),
            )
        })
    }

    pub fn path_axes_der2_mat(&self) -> &Mat33<S> {
        self.path_axes_der2_mat.get_or_init(|| {
            let (x, y, z) = *self.path_axes_der2();
            Mat33::from_col_vecs(x, y, z)
        })
    }

    pub fn path_axes_start_inverse_mat(&self) -> &Mat33<S> {
        self.path_axes_start_inverse_mat.get_or_init(|| {
            let (x, y, z) = axes(&self.path_start, self.sweep.path.never_tangent());
            let matrix = Mat33::from_col_vecs(x, y, z);
            matrix.inverse().unwrap()
        })
    }
}
impl<'a, S: Scalar> ISurfacePoint<S> for SweepPoint<'a, S> {
    fn uv(&self) -> Vec2<S> {
        self.uv
    }

    fn pos(&self) -> &Vec3<S> {
        self.eval.get_or_init(|| {
            let m = self.path_axes_mat() * self.path_axes_start_inverse_mat();
            self.path_v.pos() + m * (self.profile_u.pos() - self.path_start.pos())
        })
    }

    fn der1(&self) -> &(Vec3<S>, Vec3<S>) {
        self.der1.get_or_init(|| {
            let m = self.path_axes_mat() * self.path_axes_start_inverse_mat();
            let du = m * self.profile_u.der1();

            let m_der1 = self.path_axes_der1_mat() * self.path_axes_start_inverse_mat();
            let dv = self.path_v.der1() + m_der1 * (self.profile_u.pos() - self.path_start.pos());

            (du, dv)
        })
    }

    fn der2(&self) -> &(Vec3<S>, Vec3<S>, Vec3<S>) {
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

    fn ff1(&self) -> &(S, S, S) {
        self.ff1.get_or_init(|| ff1(self))
    }

    fn ff2(&self) -> &(S, S, S) {
        self.ff2.get_or_init(|| ff2(self))
    }

    fn normal_curvature(&self, direction: Vec2<S>) -> S {
        *self
            .normal_curvature
            .get_or_init(|| normal_curvature(self, direction))
    }

    fn mean_curvature(&self) -> S {
        *self.mean_curvature.get_or_init(|| mean_curvature(self))
    }

    fn gaussian_curvature(&self) -> S {
        *self
            .gaussian_curvature
            .get_or_init(|| gaussian_curvature(self))
    }

    fn principal_curvatures(&self) -> &(S, S) {
        self.principal_curvatures
            .get_or_init(|| principal_curvatures(self))
    }
}
