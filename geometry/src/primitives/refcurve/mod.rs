mod line;

pub use line::*;
use space::{Mat33, Point3, Vec3};

pub trait IRefCurve<'a> {
    type Point: IRefCurvePoint<'a>;

    fn domain(&self) -> (f64, f64);

    fn domain_span(&self) -> f64 {
        let (min, max) = self.domain();
        max - min
    }

    fn point(&'a self, u: f64) -> Self::Point;

    fn never_tangent(&self) -> &Vec3;
}

pub enum RefCurve {
    Line(RefLine),
}

pub trait IRefCurvePoint<'a> {
    fn u(&self) -> f64;
    fn eval(&self) -> &Point3;
    fn der1(&self) -> &Vec3;
    fn der2(&self) -> &Vec3;
    fn der3(&self) -> &Vec3;
    //fn axes(&self) -> &CurvePointAxes<'a>;
    fn axes(&'a self) -> &Mat33;
    fn axes_der1(&'a self) -> &Mat33;
    fn axes_der2(&'a self) -> &Mat33;
}

pub enum RefCurvePoint<'a> {
    Line(RefLinePoint<'a>),
}
impl<'a> RefCurvePoint<'a> {
    pub fn u(&self) -> f64 {
        match self {
            RefCurvePoint::Line(line) => line.u(),
        }
    }

    pub fn eval(&self) -> &Point3 {
        match self {
            RefCurvePoint::Line(line) => line.eval(),
        }
    }

    pub fn der1(&self) -> &Vec3 {
        match self {
            RefCurvePoint::Line(line) => line.der1(),
        }
    }

    pub fn der2(&self) -> &Vec3 {
        match self {
            RefCurvePoint::Line(line) => line.der2(),
        }
    }

    pub fn der3(&self) -> &Vec3 {
        match self {
            RefCurvePoint::Line(line) => line.der3(),
        }
    }

    pub fn axes(&'a self) -> &Mat33 {
        match self {
            RefCurvePoint::Line(line) => line.axes(),
        }
    }

    pub fn axes_der1(&'a self) -> &Mat33 {
        match self {
            RefCurvePoint::Line(line) => line.axes(),
        }
    }

    pub fn axes_der2(&'a self) -> &Mat33 {
        match self {
            RefCurvePoint::Line(line) => line.axes(),
        }
    }
}
impl<'a> From<RefLinePoint<'a>> for RefCurvePoint<'a> {
    fn from(point: RefLinePoint<'a>) -> Self {
        Self::Line(point)
    }
}
