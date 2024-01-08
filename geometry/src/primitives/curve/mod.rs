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

use crate::PrimitiveGeometry;

#[derive(Clone, Debug)]
pub enum Curve {
    Line(Line),
    Arc(Arc),
    Helix(Helix),
    SSCurve(SSCurve),
}
impl Curve {
    pub fn solver(&self, geometry: &PrimitiveGeometry) -> CurveSolver {
        match self {
            Curve::Line(line) => CurveSolver::new(line.solver(geometry).into()),
            Curve::Arc(arc) => CurveSolver::new(arc.solver(geometry).into()),
            Curve::Helix(helix) => CurveSolver::new(helix.solver(geometry).into()),
            Curve::SSCurve(ss_curve) => CurveSolver::new(ss_curve.solver(geometry).into()),
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
impl From<SSCurve> for Curve {
    fn from(ss_curve: SSCurve) -> Self {
        Self::SSCurve(ss_curve)
    }
}
