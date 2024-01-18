use super::{ICurvePoint, ICurveSolver};
use crate::{
    math::{vec3, Angle, Quat, Scalar, Vec3},
    PrimitiveGeometry,
};
use std::{cell::OnceCell, rc::Rc};

#[derive(Clone, Debug)]
pub struct Arc<S: Scalar> {
    r: S,
    angle: Angle<S>,
    orientation: Quat<S>,
    translation: Vec3<S>,
}
impl<S: Scalar> Arc<S> {
    pub fn new(r: S, angle: Angle<S>, orientation: Quat<S>, translation: Vec3<S>) -> Self {
        Self {
            r,
            angle,
            orientation,
            translation,
        }
    }

    pub fn solver(&self, _geometry: &PrimitiveGeometry<f64>) -> ArcSolver<S> {
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
pub struct ArcSolver<S: Scalar> {
    r: S,
    angle: Angle<S>,
    orientation: Quat<S>,
    translation: Vec3<S>,

    never_tangent: OnceCell<Vec3<S>>,
}
impl<S: Scalar> ArcSolver<S> {
    pub fn new(r: S, angle: Angle<S>, orientation: Quat<S>, translation: Vec3<S>) -> Self {
        Self {
            r,
            angle,
            orientation,
            translation,

            never_tangent: OnceCell::new(),
        }
    }
}
impl<S: Scalar> ICurveSolver<S> for ArcSolver<S> {
    type PointSolver = ArcPointSolver<S>;

    fn domain(&self) -> (S, S) {
        (S::ZERO, self.angle.radians())
    }

    fn point(&self, u: S) -> Self::PointSolver {
        ArcPointSolver::new(self.clone(), u)
    }

    fn never_tangent(&self) -> &Vec3<S> {
        self.never_tangent
            .get_or_init(|| self.orientation * Vec3::UNIT_Z)
    }
}

pub struct ArcPointSolver<S: Scalar> {
    inner: Rc<ArcPointInner<S>>,
}
impl<S: Scalar> ArcPointSolver<S> {
    pub fn new(arc: ArcSolver<S>, u: S) -> Self {
        Self {
            inner: Rc::new(ArcPointInner::new(arc, u)),
        }
    }
}
impl<S: Scalar> ICurvePoint<S> for ArcPointSolver<S> {
    fn u(&self) -> S {
        self.inner.u
    }

    fn pos(&self) -> &Vec3<S> {
        self.inner.eval.get_or_init(|| {
            let point = vec3(
                self.inner.arc.r * self.inner.u.cos(),
                self.inner.arc.r * self.inner.u.sin(),
                S::ZERO,
            );
            self.inner.arc.orientation * point + self.inner.arc.translation
        })
    }

    fn der1(&self) -> &Vec3<S> {
        self.inner.der1.get_or_init(|| {
            let der1 = vec3(
                self.inner.arc.r * -self.inner.u.sin(),
                self.inner.arc.r * self.inner.u.cos(),
                S::ZERO,
            );
            self.inner.arc.orientation * der1
        })
    }

    fn der2(&self) -> &Vec3<S> {
        self.inner.der2.get_or_init(|| {
            let der2 = vec3(
                self.inner.arc.r * -self.inner.u.cos(),
                self.inner.arc.r * -self.inner.u.sin(),
                S::ZERO,
            );
            self.inner.arc.orientation * der2
        })
    }

    fn der3(&self) -> &Vec3<S> {
        self.inner.der3.get_or_init(|| {
            let der3 = vec3(
                self.inner.arc.r * self.inner.u.sin(),
                self.inner.arc.r * -self.inner.u.cos(),
                S::ZERO,
            );
            self.inner.arc.orientation * der3
        })
    }

    fn curvature(&self) -> S {
        S::ONE / self.inner.arc.r
    }
}

struct ArcPointInner<S: Scalar> {
    u: S,
    arc: ArcSolver<S>,

    eval: OnceCell<Vec3<S>>,
    der1: OnceCell<Vec3<S>>,
    der2: OnceCell<Vec3<S>>,
    der3: OnceCell<Vec3<S>>,
}
impl<S: Scalar> ArcPointInner<S> {
    pub fn new(arc: ArcSolver<S>, u: S) -> Self {
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
