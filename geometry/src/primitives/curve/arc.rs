use space::{point3, vec3, Angle, Point3, Quat, Vec3};
use std::cell::OnceCell;

use super::{CurvePointAxes, ICurve, ICurvePoint};

#[derive(Clone)]
pub struct ArcCurve {
    r: f64,
    angle: Angle,
    orientation: Quat,
    translation: Vec3,

    never_tangent: OnceCell<Vec3>,
}
impl ArcCurve {
    pub fn new(r: f64, angle: Angle, orientation: Quat, translation: Vec3) -> Self {
        Self {
            r,
            angle,
            orientation,
            translation,

            never_tangent: OnceCell::new(),
        }
    }
}
impl<'a> ICurve<'a> for ArcCurve {
    type Point = ArcPoint<'a>;

    fn domain(&self) -> (f64, f64) {
        (0.0, self.angle.radians())
    }

    fn point(&'a self, u: f64) -> Self::Point {
        ArcPoint::new(self, u)
    }

    fn never_tangent(&self) -> &Vec3 {
        self.never_tangent
            .get_or_init(|| self.orientation * Vec3::UNIT_Z)
    }
}

pub struct ArcPoint<'a> {
    u: f64,
    arc: &'a ArcCurve,

    eval: OnceCell<Point3>,
    der1: OnceCell<Vec3>,
    der2: OnceCell<Vec3>,
    der3: OnceCell<Vec3>,
    axes: OnceCell<CurvePointAxes<'a>>,
}
impl<'a> ArcPoint<'a> {
    pub fn new(arc: &'a ArcCurve, u: f64) -> Self {
        Self {
            u,
            arc,

            eval: OnceCell::new(),
            der1: OnceCell::new(),
            der2: OnceCell::new(),
            der3: OnceCell::new(),
            axes: OnceCell::new(),
        }
    }

    fn axes(&'a self) -> &CurvePointAxes<'a> {
        self.axes
            .get_or_init(|| CurvePointAxes::new(self, *self.arc.never_tangent()))
    }
}
impl<'a> ICurvePoint<'a> for ArcPoint<'a> {
    fn u(&self) -> f64 {
        self.u
    }

    fn eval(&self) -> &Point3 {
        self.eval.get_or_init(|| {
            let point = point3(self.arc.r * self.u.cos(), self.arc.r * self.u.sin(), 0.0);
            self.arc.orientation * point + self.arc.translation
        })
    }

    fn der1(&self) -> &Vec3 {
        self.der1.get_or_init(|| {
            let der1 = vec3(self.arc.r * -self.u.sin(), self.arc.r * self.u.cos(), 0.0);
            self.arc.orientation * der1
        })
    }

    fn der2(&self) -> &Vec3 {
        self.der2.get_or_init(|| {
            let der2 = vec3(self.arc.r * -self.u.cos(), self.arc.r * -self.u.sin(), 0.0);
            self.arc.orientation * der2
        })
    }

    fn der3(&self) -> &Vec3 {
        self.der3.get_or_init(|| {
            let der3 = vec3(self.arc.r * self.u.sin(), self.arc.r * -self.u.cos(), 0.0);
            self.arc.orientation * der3
        })
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
