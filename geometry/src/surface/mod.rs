mod sweep;

pub use sweep::*;

use std::cell::OnceCell;

use space::{point2, Mat33, Point2, Point3, Vec2, Vec3};

use crate::{Curve3, Curve3Impl};

pub trait ISurface<'a> {
    type Point: ISurfacePoint;

    fn domain(&self) -> (Point2, Point2);

    fn domain_span(&self) -> Vec2 {
        let (min, max) = self.domain();
        max - min
    }

    fn point(&'a self, at: Point2) -> Self::Point;
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

    pub fn eval(&self) -> &Point3 {
        match self {
            SurfacePoint::Sweep(sweep) => sweep.eval(),
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

pub enum Surface {
    Sweep(SweepSurface),
}
impl Surface {
    pub fn domain(&self) -> (Point2, Point2) {
        match self {
            Surface::Sweep(sweep) => sweep.domain(),
        }
    }

    pub fn point(&self, uv: Point2) -> SurfacePoint {
        match self {
            Surface::Sweep(sweep) => SurfacePoint::from(sweep.point(uv)),
        }
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
    fn eval(&self) -> &Point3;
    fn der1(&self) -> &(Vec3, Vec3);
    fn der2(&self) -> &(Vec3, Vec3, Vec3);
    fn ff1(&self) -> &(f64, f64, f64);
    fn ff2(&self) -> &(f64, f64, f64);
    fn normal_curvature(&self, direction: Vec2) -> f64;
    fn mean_curvature(&self) -> f64;
    fn gaussian_curvature(&self) -> f64;
    fn principal_curvatures(&self) -> &(f64, f64);
}

fn ff1<P: ISurfacePoint>(point: &P) -> (f64, f64, f64) {
    let (du, dv) = point.der1();

    let e = du.dot(*du);
    let f = du.dot(*dv);
    let g = dv.dot(*dv);

    (e, f, g)
}

fn ff2<P: ISurfacePoint>(point: &P) -> (f64, f64, f64) {
    let (du, dv) = point.der1();
    let (duu, duv, dvv) = point.der2();

    let normal = dv.cross(*du).normalize();

    let l = duu.dot(normal);
    let m = duv.dot(normal);
    let n = dvv.dot(normal);

    (l, m, n)
}

fn normal_curvature<P: ISurfacePoint>(point: &P, direction: Vec2) -> f64 {
    let (e, f, g) = point.ff1();
    let (l, m, n) = point.ff2();

    let du2 = direction.u().powi(2);
    let dudv = direction.u() * direction.v();
    let dv2 = direction.v().powi(2);

    (l * du2 + 2.0 * m * dudv + n * dv2) / (e * du2 + 2.0 * f * dudv + g * dv2)
}

fn mean_curvature<P: ISurfacePoint>(point: &P) -> f64 {
    let (e, f, g) = point.ff1();
    let (l, m, n) = point.ff2();

    0.5 * (e * n - 2.0 * f * m + g * l) / (e * g - f.powi(2))
}

fn gaussian_curvature<P: ISurfacePoint>(point: &P) -> f64 {
    let (e, f, g) = point.ff1();
    let (l, m, n) = point.ff2();
    (l * n - m.powi(2)) / (e * g - f.powi(2))
}

fn principal_curvatures<P: ISurfacePoint>(point: &P) -> (f64, f64) {
    let h = point.mean_curvature();
    let k = point.gaussian_curvature();

    let root = (h.powi(2) - k).sqrt();

    (h + root, h - root)
}
