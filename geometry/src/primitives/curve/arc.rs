use super::{ICurvePoint, ICurveSolver};
use crate::{
    math::{point3, vec3, Angle, Point3, Quat, Vec3},
    PrimitiveGeometry,
};
use std::{cell::OnceCell, rc::Rc, sync};

#[derive(Clone, Debug)]
pub struct Arc {
    r: f64,
    angle: Angle,
    orientation: Quat,
    translation: Vec3,
}
impl Arc {
    pub fn new(r: f64, angle: Angle, orientation: Quat, translation: Vec3) -> Self {
        Self {
            r,
            angle,
            orientation,
            translation,
        }
    }

    pub fn solver(&self, _geometry: &PrimitiveGeometry) -> ArcSolver {
        ArcSolver {
            r: self.r,
            angle: self.angle,
            orientation: self.orientation,
            translation: self.translation,
            never_tangent: OnceCell::new(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct ArcSolver {
    r: f64,
    angle: Angle,
    orientation: Quat,
    translation: Vec3,

    never_tangent: OnceCell<Vec3>,
}
impl ArcSolver {
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
impl<'a> ICurveSolver<'a> for ArcSolver {
    type Point = ArcPoint;

    fn domain(&self) -> (f64, f64) {
        (0.0, self.angle.radians())
    }

    fn point(&'a self, u: f64) -> Self::Point {
        ArcPoint::new(self.clone(), u)
    }

    fn never_tangent(&self) -> &Vec3 {
        self.never_tangent
            .get_or_init(|| self.orientation * Vec3::UNIT_Z)
    }
}

pub struct ArcPoint {
    inner: Rc<ArcPointInner>,
}
impl ArcPoint {
    pub fn new(arc: ArcSolver, u: f64) -> Self {
        Self {
            inner: Rc::new(ArcPointInner::new(arc, u)),
        }
    }
}

struct ArcPointInner {
    u: f64,
    arc: ArcSolver,

    eval: OnceCell<Point3>,
    der1: OnceCell<Vec3>,
    der2: OnceCell<Vec3>,
    der3: OnceCell<Vec3>,
}
impl ArcPointInner {
    pub fn new(arc: ArcSolver, u: f64) -> Self {
        Self {
            u,
            arc,

            eval: OnceCell::new(),
            der1: OnceCell::new(),
            der2: OnceCell::new(),
            der3: OnceCell::new(),
        }
    }
}
impl ICurvePoint for ArcPoint {
    fn u(&self) -> f64 {
        self.inner.u
    }

    fn pos(&self) -> &Point3 {
        self.inner.eval.get_or_init(|| {
            let point = point3(
                self.inner.arc.r * self.inner.u.cos(),
                self.inner.arc.r * self.inner.u.sin(),
                0.0,
            );
            self.inner.arc.orientation * point + self.inner.arc.translation
        })
    }

    fn der1(&self) -> &Vec3 {
        self.inner.der1.get_or_init(|| {
            let der1 = vec3(
                self.inner.arc.r * -self.inner.u.sin(),
                self.inner.arc.r * self.inner.u.cos(),
                0.0,
            );
            self.inner.arc.orientation * der1
        })
    }

    fn der2(&self) -> &Vec3 {
        self.inner.der2.get_or_init(|| {
            let der2 = vec3(
                self.inner.arc.r * -self.inner.u.cos(),
                self.inner.arc.r * -self.inner.u.sin(),
                0.0,
            );
            self.inner.arc.orientation * der2
        })
    }

    fn der3(&self) -> &Vec3 {
        self.inner.der3.get_or_init(|| {
            let der3 = vec3(
                self.inner.arc.r * self.inner.u.sin(),
                self.inner.arc.r * -self.inner.u.cos(),
                0.0,
            );
            self.inner.arc.orientation * der3
        })
    }

    fn curvature(&self) -> f64 {
        1.0 / self.inner.arc.r
    }
}
