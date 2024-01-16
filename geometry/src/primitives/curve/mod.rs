mod arc;
mod curve_point;
mod curve_solver;
mod helix;
mod line;
mod ss_curve;

pub use arc::*;
pub use curve_point::*;
pub use curve_solver::*;
pub use helix::*;
pub use line::*;
pub use ss_curve::*;

use crate::{math::Scalar, PrimitiveGeometry};

#[derive(Clone, Debug)]
pub enum Curve<S: Scalar> {
    Line(Line<S>),
    Arc(Arc<S>),
    Helix(Helix<S>),
    SSCurve(SSCurve<S>),
}
impl<S: Scalar> Curve<S> {
    pub fn solver(&self, geometry: &PrimitiveGeometry<S>) -> CurveSolver<S> {
        match self {
            Curve::Line(line) => CurveSolver::new(line.solver(geometry).into()),
            Curve::Arc(arc) => CurveSolver::new(arc.solver(geometry).into()),
            Curve::Helix(helix) => CurveSolver::new(helix.solver(geometry).into()),
            Curve::SSCurve(ss_curve) => CurveSolver::new(ss_curve.solver(geometry).into()),
        }
    }
}
impl<S: Scalar> From<Line<S>> for Curve<S> {
    fn from(line: Line<S>) -> Self {
        Self::Line(line)
    }
}
impl<S: Scalar> From<Arc<S>> for Curve<S> {
    fn from(arc: Arc<S>) -> Self {
        Self::Arc(arc)
    }
}
impl<S: Scalar> From<Helix<S>> for Curve<S> {
    fn from(helix: Helix<S>) -> Self {
        Self::Helix(helix)
    }
}
impl<S: Scalar> From<SSCurve<S>> for Curve<S> {
    fn from(ss_curve: SSCurve<S>) -> Self {
        Self::SSCurve(ss_curve)
    }
}
