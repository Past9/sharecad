use super::SweepPoint;
use crate::math::{Point2, Point3, Vec2, Vec3};

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

    fn curvature_u(&self) -> f64 {
        let (du, _) = *self.der1();
        let (duu, _, _) = *self.der2();
        (du.magnitude().powi(3) / (du.cross(duu)).magnitude()).recip()
    }

    fn curvature_v(&self) -> f64 {
        let (_, dv) = *self.der1();
        let (_, _, dvv) = *self.der2();
        (dv.magnitude().powi(3) / (dv.cross(dvv)).magnitude()).recip()
    }
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

    pub fn curvature_u(&self) -> f64 {
        match self {
            SurfacePoint::Sweep(sweep) => sweep.curvature_u(),
        }
    }

    pub fn curvature_v(&self) -> f64 {
        match self {
            SurfacePoint::Sweep(sweep) => sweep.curvature_v(),
        }
    }
}
impl<'a> From<SweepPoint<'a>> for SurfacePoint<'a> {
    fn from(point: SweepPoint<'a>) -> Self {
        Self::Sweep(point)
    }
}
