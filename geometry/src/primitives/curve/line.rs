use super::{ICurvePoint, ICurveSolver};
use crate::{
    math::{vec3, Coincidence, Scalar, Vec3},
    primitives::Point,
    IGeometry, PrimitiveGeometry,
};
use common::PointId;
use std::{cell::OnceCell, marker::PhantomData, rc::Rc};

#[derive(Clone, Debug)]
pub struct Line<S: Scalar> {
    start: PointId,
    end: PointId,
    _s: PhantomData<S>,
}
impl<S: Scalar> Line<S> {
    pub fn new(start: PointId, end: PointId) -> Self {
        Self {
            start,
            end,
            _s: PhantomData,
        }
    }

    pub fn solver(&self, geometry: &PrimitiveGeometry<S>) -> LineSolver<S> {
        LineSolver {
            start: geometry.point(self.start).unwrap().to_owned(),
            end: geometry.point(self.end).unwrap().to_owned(),
            never_tangent: OnceCell::new(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct LineSolver<S: Scalar> {
    start: Point<S>,
    end: Point<S>,

    never_tangent: OnceCell<Vec3<S>>,
}
impl<S: Scalar> LineSolver<S> {
    pub fn new(start: Point<S>, end: Point<S>) -> Self {
        Self {
            start,
            end,
            never_tangent: OnceCell::new(),
        }
    }
}
impl<S: Scalar> ICurveSolver<S> for LineSolver<S> {
    type PointSolver = LinePointSolver<S>;

    fn domain(&self) -> (S, S) {
        (S::ZERO, S::ONE)
    }

    fn point(&self, u: S) -> Self::PointSolver {
        LinePointSolver::new(self.clone(), u)
    }

    fn never_tangent(&self) -> &Vec3<S> {
        self.never_tangent.get_or_init(|| {
            let tangent = (self.end.pos() - self.start.pos()).normalize();
            if tangent.z.abs().cc(S::ONE) {
                vec3(S::ZERO, tangent.z, S::ZERO)
            } else {
                vec3(-tangent.y, tangent.x, tangent.z)
            }
        })
    }
}

pub struct LinePointSolver<S: Scalar> {
    inner: Rc<LinePointInner<S>>,
}
impl<S: Scalar> LinePointSolver<S> {
    pub fn new(line: LineSolver<S>, u: S) -> Self {
        Self {
            inner: Rc::new(LinePointInner::new(line, u)),
        }
    }
}
impl<S: Scalar> ICurvePoint<S> for LinePointSolver<S> {
    fn u(&self) -> S {
        self.inner.u
    }

    fn pos(&self) -> &Vec3<S> {
        self.inner.eval.get_or_init(|| {
            (S::ONE - self.inner.u) * self.inner.line.start.pos()
                + self.inner.u * self.inner.line.end.pos()
        })
    }

    fn der1(&self) -> &Vec3<S> {
        self.inner
            .der1
            .get_or_init(|| self.inner.line.end.pos() - self.inner.line.start.pos())
    }

    fn der2(&self) -> &Vec3<S> {
        self.inner.der2.get_or_init(|| Vec3::ZERO)
    }

    fn der3(&self) -> &Vec3<S> {
        self.inner.der3.get_or_init(|| Vec3::ZERO)
    }

    fn curvature(&self) -> S {
        S::ZERO
    }
}

struct LinePointInner<S: Scalar> {
    u: S,
    line: LineSolver<S>,

    eval: OnceCell<Vec3<S>>,
    der1: OnceCell<Vec3<S>>,
    der2: OnceCell<Vec3<S>>,
    der3: OnceCell<Vec3<S>>,
}
impl<S: Scalar> LinePointInner<S> {
    pub fn new(line: LineSolver<S>, u: S) -> Self {
        Self {
            line,
            u,

            eval: OnceCell::new(),
            der1: OnceCell::new(),
            der2: OnceCell::new(),
            der3: OnceCell::new(),
        }
    }
}
