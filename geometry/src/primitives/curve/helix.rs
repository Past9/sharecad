use crate::Geometry;

use super::{CurvePointAxes, ICurvePoint, ICurveSolver};
use space::{point3, vec3, Point3, Quat, Vec3};
use std::{cell::OnceCell, f64::consts::TAU};

#[derive(Clone)]
pub struct Helix {
    r: f64,
    h: f64,
    n: f64,
    orientation: Quat,
    translation: Vec3,
}
impl Helix {
    pub fn solver(&self, _geometry: &Geometry) -> HelixSolver {
        HelixSolver {
            r: self.r,
            h: self.h,
            n: self.n,
            orientation: self.orientation,
            translation: self.translation,
            never_tangent: OnceCell::new(),
        }
    }
}

#[derive(Clone)]
pub struct HelixSolver {
    /// Radius of the helix
    r: f64,
    /// Axial length (not length along the helical curve)
    /// of one complete revolution of the helix multiplied by 2PI
    h: f64,
    /// Number of revolutions of the helix
    n: f64,

    orientation: Quat,
    translation: Vec3,

    never_tangent: OnceCell<Vec3>,
}
impl HelixSolver {
    pub fn new(r: f64, h: f64, n: f64, orientation: Quat, translation: Vec3) -> Self {
        Self {
            r,
            h,
            n,
            orientation,
            translation,

            never_tangent: OnceCell::new(),
        }
    }

    /// Number of revolutions
    pub fn n(&self) -> f64 {
        self.n
    }

    /// Radius
    pub fn r(&self) -> f64 {
        self.r
    }

    pub fn arc_len(&self, u: f64) -> f64 {
        (self.h.powi(2) + self.r.powi(2)).sqrt() * u
    }
}
impl<'a> ICurveSolver<'a> for HelixSolver {
    type Point = HelixPoint<'a>;

    fn domain(&self) -> (f64, f64) {
        (0.0, self.n * TAU)
    }

    fn point(&'a self, u: f64) -> Self::Point {
        HelixPoint::new(self, u)
    }

    fn never_tangent(&self) -> &Vec3 {
        self.never_tangent
            .get_or_init(|| self.orientation * Vec3::UNIT_Z)
    }
}

pub struct HelixPoint<'a> {
    u: f64,
    helix: &'a HelixSolver,

    eval: OnceCell<Point3>,
    der1: OnceCell<Vec3>,
    der2: OnceCell<Vec3>,
    der3: OnceCell<Vec3>,
    axes: OnceCell<CurvePointAxes<'a>>,
}
impl<'a> HelixPoint<'a> {
    pub fn new(helix: &'a HelixSolver, u: f64) -> Self {
        Self {
            u,
            helix,

            eval: OnceCell::new(),
            der1: OnceCell::new(),
            der2: OnceCell::new(),
            der3: OnceCell::new(),
            axes: OnceCell::new(),
        }
    }

    fn axes(&'a self) -> &CurvePointAxes<'a> {
        self.axes
            .get_or_init(|| CurvePointAxes::new(self, *self.helix.never_tangent()))
    }
}
impl<'a> ICurvePoint<'a> for HelixPoint<'a> {
    fn u(&self) -> f64 {
        self.u
    }

    fn eval(&self) -> &Point3 {
        self.eval.get_or_init(|| {
            let point = point3(
                self.helix.r * self.u.cos(),
                self.helix.r * self.u.sin(),
                self.helix.h * self.u,
            );
            self.helix.orientation * point + self.helix.translation
        })
    }

    fn der1(&self) -> &Vec3 {
        self.der1.get_or_init(|| {
            let der1 = vec3(
                self.helix.r * -self.u.sin(),
                self.helix.r * self.u.cos(),
                self.helix.h,
            );
            self.helix.orientation * der1
        })
    }

    fn der2(&self) -> &Vec3 {
        self.der2.get_or_init(|| {
            let der2 = vec3(
                self.helix.r * -self.u.cos(),
                self.helix.r * -self.u.sin(),
                0.0,
            );
            self.helix.orientation * der2
        })
    }

    fn der3(&self) -> &Vec3 {
        self.der3.get_or_init(|| {
            let der3 = vec3(
                self.helix.r * self.u.sin(),
                self.helix.r * -self.u.cos(),
                0.0,
            );
            self.helix.orientation * der3
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
