use super::{ICurvePoint, ICurveSolver};
use crate::{
    math::{vec3, Interval, Quat, Scalar, Vec3},
    PrimitiveGeometry,
};
use std::{cell::OnceCell, f64::consts::TAU, rc::Rc};

#[derive(Clone, Debug)]
pub struct Helix<S: Scalar> {
    r: S,
    h: S,
    n: S,
    orientation: Quat<S>,
    translation: Vec3<S>,
}
impl<S: Scalar> Helix<S> {
    pub fn new(r: S, h: S, n: S, orientation: Quat<S>, translation: Vec3<S>) -> Self {
        Self {
            r,
            h,
            n,
            orientation,
            translation,
        }
    }

    pub fn solver(&self, _geometry: &PrimitiveGeometry<S>) -> HelixSolver<S> {
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
impl HelixSolver<f64> {
    pub fn as_interval(&self) -> HelixSolver<Interval> {
        HelixSolver {
            r: Interval::thin(self.r),
            h: Interval::thin(self.h),
            n: Interval::thin(self.n),
            orientation: self.orientation.as_interval(),
            translation: self.translation.as_interval(),
            never_tangent: OnceCell::from(self.never_tangent().as_interval()),
        }
    }
}

#[derive(Clone, Debug)]
pub struct HelixSolver<S: Scalar> {
    /// Radius of the helix
    r: S,
    /// Axial length (not length along the helical curve)
    /// of one complete revolution of the helix multiplied by 2PI
    h: S,
    /// Number of revolutions of the helix
    n: S,

    orientation: Quat<S>,
    translation: Vec3<S>,

    never_tangent: OnceCell<Vec3<S>>,
}
impl<S: Scalar> HelixSolver<S> {
    pub fn new(r: S, h: S, n: S, orientation: Quat<S>, translation: Vec3<S>) -> Self {
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
    pub fn n(&self) -> S {
        self.n
    }

    /// Radius
    pub fn r(&self) -> S {
        self.r
    }

    pub fn arc_len(&self, u: S) -> S {
        (self.h.powi(2) + self.r.powi(2)).sqrt() * u
    }
}
impl<S: Scalar> ICurveSolver<S> for HelixSolver<S> {
    type PointSolver = HelixPointSolver<S>;

    fn domain(&self) -> (S, S) {
        (S::ZERO, self.n * S::TAU)
    }

    fn point(&self, u: S) -> Self::PointSolver {
        HelixPointSolver::new(self.clone(), u)
    }

    fn never_tangent(&self) -> &Vec3<S> {
        self.never_tangent
            .get_or_init(|| self.orientation * Vec3::UNIT_Z)
    }
}

pub struct HelixPointSolver<S: Scalar> {
    inner: Rc<HelixPointInner<S>>,
}
impl<S: Scalar> HelixPointSolver<S> {
    pub fn new(helix: HelixSolver<S>, u: S) -> Self {
        Self {
            inner: Rc::new(HelixPointInner::new(helix, u)),
        }
    }
}
impl<S: Scalar> ICurvePoint<S> for HelixPointSolver<S> {
    fn u(&self) -> S {
        self.inner.u
    }

    fn pos(&self) -> &Vec3<S> {
        self.inner.eval.get_or_init(|| {
            let point = vec3(
                self.inner.helix.r * self.inner.u.cos(),
                self.inner.helix.r * self.inner.u.sin(),
                self.inner.helix.h * self.inner.u,
            );
            self.inner.helix.orientation * point + self.inner.helix.translation
        })
    }

    fn der1(&self) -> &Vec3<S> {
        self.inner.der1.get_or_init(|| {
            let der1 = vec3(
                self.inner.helix.r * -self.inner.u.sin(),
                self.inner.helix.r * self.inner.u.cos(),
                self.inner.helix.h,
            );
            self.inner.helix.orientation * der1
        })
    }

    fn der2(&self) -> &Vec3<S> {
        self.inner.der2.get_or_init(|| {
            let der2 = vec3(
                self.inner.helix.r * -self.inner.u.cos(),
                self.inner.helix.r * -self.inner.u.sin(),
                S::ZERO,
            );
            self.inner.helix.orientation * der2
        })
    }

    fn der3(&self) -> &Vec3<S> {
        self.inner.der3.get_or_init(|| {
            let der3 = vec3(
                self.inner.helix.r * self.inner.u.sin(),
                self.inner.helix.r * -self.inner.u.cos(),
                S::ZERO,
            );
            self.inner.helix.orientation * der3
        })
    }
}

struct HelixPointInner<S: Scalar> {
    u: S,
    helix: HelixSolver<S>,

    eval: OnceCell<Vec3<S>>,
    der1: OnceCell<Vec3<S>>,
    der2: OnceCell<Vec3<S>>,
    der3: OnceCell<Vec3<S>>,
}
impl<S: Scalar> HelixPointInner<S> {
    fn new(helix: HelixSolver<S>, u: S) -> Self {
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
