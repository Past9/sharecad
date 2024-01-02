use std::cell::OnceCell;

use common::{CurveId, PointId};
use space::{Mat33, Point3, Vec3};

use crate::{primitives::CurvePointAxes, RefGeometry};

use super::{IRefCurvePoint, RefCurvePoint};

pub struct RefLine {
    id: CurveId,
    start: PointId,
    end: PointId,
}
impl RefLine {
    pub fn new(id: CurveId, start: PointId, end: PointId) -> Self {
        Self { id, start, end }
    }
}

pub struct RefLineSolver {
    start: Point3,
    end: Point3,
    never_tangent: OnceCell<Vec3>,
}
impl RefLineSolver {
    pub fn point(&self, u: f64) -> RefLinePoint {
        RefLinePoint {
            u,
            line: self,
            eval: OnceCell::new(),
            der1: OnceCell::new(),
            der2: OnceCell::new(),
            der3: OnceCell::new(),
            axes: OnceCell::new(),
        }
    }

    pub fn never_tangent(&self) -> RefLine
}

pub struct RefLinePoint<'a> {
    u: f64,
    line: &'a RefLineSolver,

    eval: OnceCell<Point3>,
    der1: OnceCell<Vec3>,
    der2: OnceCell<Vec3>,
    der3: OnceCell<Vec3>,
    axes: OnceCell<RefCurvePointAxes<'a>>,
}
impl<'a> RefLinePoint<'a> {
    fn axes(&'a self) -> &RefCurvePointAxes<'a> {
        self.axes
            .get_or_init(|| RefCurvePointAxes::new(self, *self.line.never_tangent()))
    }
}
impl<'a> IRefCurvePoint<'a> for RefLinePoint<'a> {
    fn u(&self) -> f64 {
        self.u
    }

    fn eval(&self) -> &Point3 {
        self.eval
            .get_or_init(|| ((1.0 - self.u) * self.start + self.u * self.end))
    }

    fn der1(&self) -> &Vec3 {
        self.der1.get_or_init(|| self.end - self.start)
    }

    fn der2(&self) -> &Vec3 {
        self.der2.get_or_init(|| Vec3::ZERO)
    }

    fn der3(&self) -> &Vec3 {
        self.der3.get_or_init(|| Vec3::ZERO)
    }

    fn axes(&'a self) -> &space::Mat33 {
        self.axes().axes_mat()
    }

    fn axes_der1(&'a self) -> &space::Mat33 {
        self.axes().axes_der1_mat()
    }

    fn axes_der2(&'a self) -> &space::Mat33 {
        self.axes().axes_der2_mat()
    }
}

pub struct RefCurvePointAxes<'a> {
    point: &'a dyn IRefCurvePoint<'a>,
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
impl<'a> RefCurvePointAxes<'a> {
    fn new(point: &'a impl IRefCurvePoint<'a>, never_tangent: Vec3) -> Self {
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

fn axes<'a>(point: &RefCurvePoint<'a>, never_tangent: &Vec3) -> (Vec3, Vec3, Vec3) {
    let i1 = point.der1().normalize();
    let d = *never_tangent;

    let d2 = d - (i1.dot(d)) * i1;

    let i2 = d2.normalize();
    let i3 = i1.cross(i2);

    (i1, i2, i3)
}

fn axes_der1<'a>(
    point: &RefCurvePoint<'a>,
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

fn axes_der2<'a>(
    point: &RefCurvePoint<'a>,
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
