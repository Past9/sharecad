use super::SweepPoint;
use crate::math::{Scalar, Vec2, Vec3};

pub trait ISurfacePoint<S: Scalar> {
    fn u(&self) -> S {
        self.uv().u()
    }

    fn v(&self) -> S {
        self.uv().v()
    }

    fn uv(&self) -> Vec2<S>;
    fn pos(&self) -> &Vec3<S>;
    fn der1(&self) -> &(Vec3<S>, Vec3<S>);
    fn der2(&self) -> &(Vec3<S>, Vec3<S>, Vec3<S>);
    fn ff1(&self) -> &(S, S, S);
    fn ff2(&self) -> &(S, S, S);
    fn normal_curvature(&self, direction: Vec2<S>) -> S;
    fn mean_curvature(&self) -> S;
    fn gaussian_curvature(&self) -> S;
    fn principal_curvatures(&self) -> &(S, S);

    fn curvature_u(&self) -> S {
        let (du, _) = *self.der1();
        let (duu, _, _) = *self.der2();
        (du.magnitude().powi(3) / (du.cross(duu)).magnitude()).recip()
    }

    fn curvature_v(&self) -> S {
        let (_, dv) = *self.der1();
        let (_, _, dvv) = *self.der2();
        (dv.magnitude().powi(3) / (dv.cross(dvv)).magnitude()).recip()
    }
}

pub enum SurfacePoint<'a, S: Scalar> {
    Sweep(SweepPoint<'a, S>),
}
impl<'a, S: Scalar> ISurfacePoint<S> for SurfacePoint<'a, S> {
    fn u(&self) -> S {
        match self {
            SurfacePoint::Sweep(sweep) => sweep.u(),
        }
    }

    fn v(&self) -> S {
        match self {
            SurfacePoint::Sweep(sweep) => sweep.v(),
        }
    }

    fn uv(&self) -> Vec2<S> {
        match self {
            SurfacePoint::Sweep(sweep) => sweep.uv(),
        }
    }

    fn pos(&self) -> &Vec3<S> {
        match self {
            SurfacePoint::Sweep(sweep) => sweep.pos(),
        }
    }

    fn der1(&self) -> &(Vec3<S>, Vec3<S>) {
        match self {
            SurfacePoint::Sweep(sweep) => sweep.der1(),
        }
    }

    fn der2(&self) -> &(Vec3<S>, Vec3<S>, Vec3<S>) {
        match self {
            SurfacePoint::Sweep(sweep) => sweep.der2(),
        }
    }

    fn ff1(&self) -> &(S, S, S) {
        match self {
            SurfacePoint::Sweep(sweep) => sweep.ff1(),
        }
    }

    fn ff2(&self) -> &(S, S, S) {
        match self {
            SurfacePoint::Sweep(sweep) => sweep.ff2(),
        }
    }

    fn normal_curvature(&self, direction: Vec2<S>) -> S {
        match self {
            SurfacePoint::Sweep(sweep) => sweep.normal_curvature(direction),
        }
    }

    fn mean_curvature(&self) -> S {
        match self {
            SurfacePoint::Sweep(sweep) => sweep.mean_curvature(),
        }
    }

    fn gaussian_curvature(&self) -> S {
        match self {
            SurfacePoint::Sweep(sweep) => sweep.gaussian_curvature(),
        }
    }

    fn principal_curvatures(&self) -> &(S, S) {
        match self {
            SurfacePoint::Sweep(sweep) => sweep.principal_curvatures(),
        }
    }

    fn curvature_u(&self) -> S {
        match self {
            SurfacePoint::Sweep(sweep) => sweep.curvature_u(),
        }
    }

    fn curvature_v(&self) -> S {
        match self {
            SurfacePoint::Sweep(sweep) => sweep.curvature_v(),
        }
    }
}
impl<'a, S: Scalar> From<SweepPoint<'a, S>> for SurfacePoint<'a, S> {
    fn from(point: SweepPoint<'a, S>) -> Self {
        Self::Sweep(point)
    }
}
