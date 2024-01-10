use super::{ICurvePoint, ICurveSolver};
use crate::{
    math::{point3, vec3, Point3, Quat, Vec3},
    PrimitiveGeometry,
};
use std::{cell::OnceCell, f64::consts::TAU, rc::Rc};

#[derive(Clone, Debug)]
pub struct Helix {
    r: f64,
    h: f64,
    n: f64,
    orientation: Quat,
    translation: Vec3,
}
impl Helix {
    pub fn new(r: f64, h: f64, n: f64, orientation: Quat, translation: Vec3) -> Self {
        Self {
            r,
            h,
            n,
            orientation,
            translation,
        }
    }

    pub fn solver(&self, _geometry: &PrimitiveGeometry) -> HelixSolver {
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

#[derive(Clone, Debug)]
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
impl ICurveSolver for HelixSolver {
    type PointSolver = HelixPointSolver;

    fn domain(&self) -> (f64, f64) {
        (0.0, self.n * TAU)
    }

    fn point(&self, u: f64) -> Self::PointSolver {
        HelixPointSolver::new(self.clone(), u)
    }

    fn never_tangent(&self) -> &Vec3 {
        self.never_tangent
            .get_or_init(|| self.orientation * Vec3::UNIT_Z)
    }
}

pub struct HelixPointSolver {
    inner: Rc<HelixPointInner>,
}
impl HelixPointSolver {
    pub fn new(helix: HelixSolver, u: f64) -> Self {
        Self {
            inner: Rc::new(HelixPointInner::new(helix, u)),
        }
    }
}
impl ICurvePoint for HelixPointSolver {
    fn u(&self) -> f64 {
        self.inner.u
    }

    fn pos(&self) -> &Point3 {
        self.inner.eval.get_or_init(|| {
            let point = point3(
                self.inner.helix.r * self.inner.u.cos(),
                self.inner.helix.r * self.inner.u.sin(),
                self.inner.helix.h * self.inner.u,
            );
            self.inner.helix.orientation * point + self.inner.helix.translation
        })
    }

    fn der1(&self) -> &Vec3 {
        self.inner.der1.get_or_init(|| {
            let der1 = vec3(
                self.inner.helix.r * -self.inner.u.sin(),
                self.inner.helix.r * self.inner.u.cos(),
                self.inner.helix.h,
            );
            self.inner.helix.orientation * der1
        })
    }

    fn der2(&self) -> &Vec3 {
        self.inner.der2.get_or_init(|| {
            let der2 = vec3(
                self.inner.helix.r * -self.inner.u.cos(),
                self.inner.helix.r * -self.inner.u.sin(),
                0.0,
            );
            self.inner.helix.orientation * der2
        })
    }

    fn der3(&self) -> &Vec3 {
        self.inner.der3.get_or_init(|| {
            let der3 = vec3(
                self.inner.helix.r * self.inner.u.sin(),
                self.inner.helix.r * -self.inner.u.cos(),
                0.0,
            );
            self.inner.helix.orientation * der3
        })
    }
}

struct HelixPointInner {
    u: f64,
    helix: HelixSolver,

    eval: OnceCell<Point3>,
    der1: OnceCell<Vec3>,
    der2: OnceCell<Vec3>,
    der3: OnceCell<Vec3>,
}
impl HelixPointInner {
    fn new(helix: HelixSolver, u: f64) -> Self {
        Self {
            u,
            helix,

            eval: OnceCell::new(),
            der1: OnceCell::new(),
            der2: OnceCell::new(),
            der3: OnceCell::new(),
        }
    }
}
