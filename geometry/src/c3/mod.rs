mod arc;
mod helix;
mod line;

pub use arc::*;
pub use helix::*;
pub use line::*;

use space::{Point3, Vec3};

pub trait ICurve<'a> {
    type Point: ICurvePoint;

    fn domain(&self) -> (f64, f64);

    fn domain_span(&self) -> f64 {
        let (min, max) = self.domain();
        max - min
    }

    fn point(&'a self, u: f64) -> Self::Point;
}

pub enum Curve {
    Line(LineCurve),
    Arc(ArcCurve),
    Helix(HelixCurve),
}
impl Curve {
    pub fn domain(&self) -> (f64, f64) {
        match self {
            Curve::Line(line) => line.domain(),
            Curve::Helix(helix) => helix.domain(),
            Curve::Arc(arc) => arc.domain(),
        }
    }

    pub fn point(&self, u: f64) -> CurvePoint {
        match self {
            Curve::Line(line) => CurvePoint::from(line.point(u)),
            Curve::Helix(helix) => CurvePoint::from(helix.point(u)),
            Curve::Arc(arc) => CurvePoint::from(arc.point(u)),
        }
    }
}

pub trait ICurvePoint {
    fn u(&self) -> f64;
    fn eval(&self) -> &Point3;
    fn der1(&self) -> &Vec3;
    fn der2(&self) -> &Vec3;
    fn der3(&self) -> &Vec3;
    fn never_tangent(&self) -> &Vec3;
}

pub enum CurvePoint<'a> {
    Line(LinePoint<'a>),
    Helix(HelixPoint<'a>),
    Arc(ArcPoint<'a>),
}
impl<'a> CurvePoint<'a> {}
impl<'a> From<LinePoint<'a>> for CurvePoint<'a> {
    fn from(point: LinePoint<'a>) -> Self {
        Self::Line(point)
    }
}
impl<'a> From<HelixPoint<'a>> for CurvePoint<'a> {
    fn from(point: HelixPoint<'a>) -> Self {
        Self::Helix(point)
    }
}
impl<'a> From<ArcPoint<'a>> for CurvePoint<'a> {
    fn from(point: ArcPoint<'a>) -> Self {
        Self::Arc(point)
    }
}
