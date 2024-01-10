use common::CurveId;

use crate::math::Point3;

#[derive(Debug, Clone)]
pub enum CCPointKind {
    Cross,
    Osculating,
}

#[derive(Debug, Clone)]
pub struct CCPoint {
    kind: CCPointKind,
    pos: Point3,
    c0: CurveId,
    c1: CurveId,
    c0_u: f64,
    c1_u: f64,
}
impl CCPoint {
    pub fn new(
        kind: CCPointKind,
        pos: Point3,
        c0: CurveId,
        c1: CurveId,
        c0_u: f64,
        c1_u: f64,
    ) -> Self {
        Self {
            kind,
            pos,
            c0,
            c1,
            c0_u,
            c1_u,
        }
    }
}
