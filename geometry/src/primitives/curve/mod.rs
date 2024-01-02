mod arc;
mod helix;
mod line;

use space::{Angle, Mat33, Point3, Quat, Vec3};
use std::cell::OnceCell;

pub use arc::*;
pub use helix::*;
pub use line::*;

use crate::Geometry;

#[derive(Clone, Debug)]
pub enum Curve {
    Line(Line),
    Arc(Arc),
    Helix(Helix),
}
impl Curve {
    pub fn solver(&self, geometry: &Geometry) -> CurveSolver {
        match self {
            Curve::Line(line) => CurveSolver::Line(line.solver(geometry)),
            Curve::Arc(arc) => CurveSolver::Arc(arc.solver(geometry)),
            Curve::Helix(helix) => CurveSolver::Helix(helix.solver(geometry)),
        }
    }
}
impl From<Line> for Curve {
    fn from(line: Line) -> Self {
        Self::Line(line)
    }
}
impl From<Arc> for Curve {
    fn from(arc: Arc) -> Self {
        Self::Arc(arc)
    }
}
impl From<Helix> for Curve {
    fn from(helix: Helix) -> Self {
        Self::Helix(helix)
    }
}

pub trait ICurveSolver<'a> {
    type Point: ICurvePoint<'a>;

    fn domain(&self) -> (f64, f64);

    fn domain_span(&self) -> f64 {
        let (min, max) = self.domain();
        max - min
    }

    fn point(&'a self, u: f64) -> Self::Point;

    fn never_tangent(&self) -> &Vec3;
}

#[derive(Clone)]
pub enum CurveSolver {
    Line(LineSolver),
    Arc(ArcSolver),
    Helix(HelixSolver),
}
impl CurveSolver {
    pub fn line(start: Point3, end: Point3) -> Self {
        LineSolver::new(start, end).into()
    }

    pub fn arc(r: f64, angle: Angle, orientation: Quat, translation: Vec3) -> Self {
        ArcSolver::new(r, angle, orientation, translation).into()
    }

    pub fn helix(r: f64, h: f64, n: f64, orientation: Quat, translation: Vec3) -> Self {
        HelixSolver::new(r, h, n, orientation, translation).into()
    }

    pub fn domain(&self) -> (f64, f64) {
        match self {
            CurveSolver::Line(line) => line.domain(),
            CurveSolver::Helix(helix) => helix.domain(),
            CurveSolver::Arc(arc) => arc.domain(),
        }
    }

    pub fn point(&self, u: f64) -> CurvePoint {
        match self {
            CurveSolver::Line(line) => CurvePoint::from(line.point(u)),
            CurveSolver::Helix(helix) => CurvePoint::from(helix.point(u)),
            CurveSolver::Arc(arc) => CurvePoint::from(arc.point(u)),
        }
    }

    pub fn never_tangent(&self) -> &Vec3 {
        match self {
            CurveSolver::Line(line) => line.never_tangent(),
            CurveSolver::Helix(helix) => helix.never_tangent(),
            CurveSolver::Arc(arc) => arc.never_tangent(),
        }
    }
}
impl From<LineSolver> for CurveSolver {
    fn from(line: LineSolver) -> Self {
        Self::Line(line)
    }
}
impl From<ArcSolver> for CurveSolver {
    fn from(arc: ArcSolver) -> Self {
        Self::Arc(arc)
    }
}
impl From<HelixSolver> for CurveSolver {
    fn from(helix: HelixSolver) -> Self {
        Self::Helix(helix)
    }
}

pub trait ICurvePoint<'a> {
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

pub enum CurvePoint<'a> {
    Line(LinePoint<'a>),
    Helix(HelixPoint<'a>),
    Arc(ArcPoint<'a>),
}
impl<'a> CurvePoint<'a> {
    pub fn u(&self) -> f64 {
        match self {
            CurvePoint::Line(line) => line.u(),
            CurvePoint::Helix(helix) => helix.u(),
            CurvePoint::Arc(arc) => arc.u(),
        }
    }

    pub fn eval(&self) -> &Point3 {
        match self {
            CurvePoint::Line(line) => line.eval(),
            CurvePoint::Helix(helix) => helix.eval(),
            CurvePoint::Arc(arc) => arc.eval(),
        }
    }

    pub fn der1(&self) -> &Vec3 {
        match self {
            CurvePoint::Line(line) => line.der1(),
            CurvePoint::Helix(helix) => helix.der1(),
            CurvePoint::Arc(arc) => arc.der1(),
        }
    }

    pub fn der2(&self) -> &Vec3 {
        match self {
            CurvePoint::Line(line) => line.der2(),
            CurvePoint::Helix(helix) => helix.der2(),
            CurvePoint::Arc(arc) => arc.der2(),
        }
    }

    pub fn der3(&self) -> &Vec3 {
        match self {
            CurvePoint::Line(line) => line.der3(),
            CurvePoint::Helix(helix) => helix.der3(),
            CurvePoint::Arc(arc) => arc.der3(),
        }
    }

    pub fn axes(&'a self) -> &Mat33 {
        match self {
            CurvePoint::Line(line) => line.axes(),
            CurvePoint::Helix(helix) => helix.axes(),
            CurvePoint::Arc(arc) => arc.axes(),
        }
    }

    pub fn axes_der1(&'a self) -> &Mat33 {
        match self {
            CurvePoint::Line(line) => line.axes(),
            CurvePoint::Helix(helix) => helix.axes(),
            CurvePoint::Arc(arc) => arc.axes(),
        }
    }

    pub fn axes_der2(&'a self) -> &Mat33 {
        match self {
            CurvePoint::Line(line) => line.axes(),
            CurvePoint::Helix(helix) => helix.axes(),
            CurvePoint::Arc(arc) => arc.axes(),
        }
    }
}
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

pub struct CurvePointAxes<'a> {
    point: &'a dyn ICurvePoint<'a>,
    never_tangent: Vec3,

    d2: OnceCell<Vec3>,
    d2_der1: OnceCell<Vec3>,

    axes: OnceCell<(Vec3, Vec3, Vec3)>,
    axes_mat: OnceCell<Mat33>,

    axes_der1: OnceCell<(Vec3, Vec3, Vec3)>,
    axes_der1_mat: OnceCell<Mat33>,

    axes_der2: OnceCell<(Vec3, Vec3, Vec3)>,
    axes_der2_mat: OnceCell<Mat33>,
}
impl<'a> CurvePointAxes<'a> {
    fn new(point: &'a impl ICurvePoint<'a>, never_tangent: Vec3) -> Self {
        Self {
            point,
            never_tangent,
            d2: OnceCell::new(),
            d2_der1: OnceCell::new(),

            axes: OnceCell::new(),
            axes_mat: OnceCell::new(),

            axes_der1: OnceCell::new(),
            axes_der1_mat: OnceCell::new(),

            axes_der2: OnceCell::new(),
            axes_der2_mat: OnceCell::new(),
        }
    }

    pub fn axes_mat(&self) -> &Mat33 {
        self.axes_mat.get_or_init(|| {
            let (x, y, z) = *self.axes();
            Mat33::from_axes(x, y, z)
        })
    }

    pub fn axes_der1_mat(&self) -> &Mat33 {
        self.axes_mat.get_or_init(|| {
            let (x, y, z) = *self.axes_der1();
            Mat33::from_axes(x, y, z)
        })
    }

    pub fn axes_der2_mat(&self) -> &Mat33 {
        self.axes_mat.get_or_init(|| {
            let (x, y, z) = *self.axes_der2();
            Mat33::from_axes(x, y, z)
        })
    }

    pub fn axes(&self) -> &(Vec3, Vec3, Vec3) {
        self.axes.get_or_init(|| {
            let i1 = self.point.der1().normalize();
            let d = self.never_tangent;

            let d2 = d - (i1.dot(d)) * i1;

            let i2 = d2.normalize();
            let i3 = i1.cross(i2);

            (i1, i2, i3)
        })
    }

    pub fn axes_der1(&self) -> &(Vec3, Vec3, Vec3) {
        self.axes_der1.get_or_init(|| {
            let (i1, i2, _) = *self.axes();

            let d = self.never_tangent;
            let d2 = d - (i1.dot(d)) * i1;

            let der1 = self.point.der1();
            let der2 = *self.point.der2();
            let i1_der1 = der1.norm_der1(der2);

            //let d2_der1 = -i1 * (i1_der1.dot(d));
            let d2_der1 = (-i1_der1.dot(d) * i1) - (i1.dot(d) * i1_der1);
            let i2_der1 = d2.norm_der1(d2_der1);

            let i3_der1 = i1.cross(i2_der1) + i1_der1.cross(i2);

            (i1_der1, i2_der1, i3_der1)
        })
    }

    pub fn axes_der2(&self) -> &(Vec3, Vec3, Vec3) {
        self.axes_der2.get_or_init(|| {
            let (i1, i2, _) = *self.axes();
            let (i1_der1, i2_der1, _) = *self.axes_der1();

            let d = self.never_tangent;
            let d2 = d - (i1.dot(d)) * i1;
            let d2_der1 = (-i1_der1.dot(d) * i1) - (i1.dot(d) * i1_der1);

            let der1 = *self.point.der1();
            let der2 = *self.point.der2();
            let der3 = *self.point.der3();

            let i1_der2 = der1.norm_der2(der2, der3);

            //let d2_der2 = -i1 * (i1_der2.dot(d));
            let d2_der2 =
                (-i1_der2.dot(d) * i1) - 2.0 * (i1_der1.dot(d) * i1_der1) - (i1.dot(d) * i1_der2);
            let i2_der2 = d2.norm_der2(d2_der1, d2_der2);

            let i3_der2 = i1.cross(i2_der2) + 2.0 * i1_der1.cross(i2_der1) + i1_der2.cross(i2);

            (i1_der2, i2_der2, i3_der2)
        })
    }
}

pub fn axes<'a>(point: &CurvePoint<'a>, never_tangent: &Vec3) -> (Vec3, Vec3, Vec3) {
    let i1 = point.der1().normalize();
    let d = *never_tangent;

    let d2 = d - (i1.dot(d)) * i1;

    let i2 = d2.normalize();
    let i3 = i1.cross(i2);

    (i1, i2, i3)
}

pub fn axes_der1<'a>(
    point: &CurvePoint<'a>,
    never_tangent: &Vec3,
    axes: &(Vec3, Vec3, Vec3),
) -> (Vec3, Vec3, Vec3) {
    let (i1, i2, _) = *axes;

    let d = *never_tangent;
    let d2 = d - (i1.dot(d)) * i1;

    let der1 = point.der1();
    let der2 = *point.der2();
    let i1_der1 = der1.norm_der1(der2);

    //let d2_der1 = -i1 * (i1_der1.dot(d));
    let d2_der1 = (-i1_der1.dot(d) * i1) - (i1.dot(d) * i1_der1);
    let i2_der1 = d2.norm_der1(d2_der1);

    let i3_der1 = i1.cross(i2_der1) + i1_der1.cross(i2);

    (i1_der1, i2_der1, i3_der1)
}

pub fn axes_der2<'a>(
    point: &CurvePoint<'a>,
    never_tangent: &Vec3,
    axes: &(Vec3, Vec3, Vec3),
    axes_der1: &(Vec3, Vec3, Vec3),
) -> (Vec3, Vec3, Vec3) {
    let (i1, i2, _) = *axes;
    let (i1_der1, i2_der1, _) = *axes_der1;

    let d = *never_tangent;
    let d2 = d - (i1.dot(d)) * i1;
    let d2_der1 = (-i1_der1.dot(d) * i1) - (i1.dot(d) * i1_der1);

    let der1 = *point.der1();
    let der2 = *point.der2();
    let der3 = *point.der3();

    let i1_der2 = der1.norm_der2(der2, der3);

    //let d2_der2 = -i1 * (i1_der2.dot(d));
    let d2_der2 = (-i1_der2.dot(d) * i1) - 2.0 * (i1_der1.dot(d) * i1_der1) - (i1.dot(d) * i1_der2);
    let i2_der2 = d2.norm_der2(d2_der1, d2_der2);

    let i3_der2 = i1.cross(i2_der2) + 2.0 * i1_der1.cross(i2_der1) + i1_der2.cross(i2);

    (i1_der2, i2_der2, i3_der2)
}
