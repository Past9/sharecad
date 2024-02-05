use common::CurveId;

use crate::math::{Scalar, Vec3};

#[derive(Debug, Clone)]
pub enum CCPointKind {
    Cross,
    Osculating,
}

#[derive(Debug, Clone)]
pub struct CCPoint<S: Scalar> {
    pub kind: CCPointKind,
    pub pos: Vec3<S>,
    pub c0: CurveId,
    pub c1: CurveId,
    pub c0_u: S,
    pub c1_u: S,
}
impl<S: Scalar> CCPoint<S> {
    pub fn new(
        kind: CCPointKind,
        pos: Vec3<S>,
        c0: CurveId,
        c1: CurveId,
        c0_u: S,
        c1_u: S,
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
