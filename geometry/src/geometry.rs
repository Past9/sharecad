use std::collections::HashMap;

use space::Point3;

use crate::primitives::{Curve, Surface};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct SurfaceId(pub u32);
impl From<u32> for SurfaceId {
    fn from(id: u32) -> Self {
        SurfaceId(id)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct CurveId(pub u32);
impl From<u32> for CurveId {
    fn from(id: u32) -> Self {
        CurveId(id)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct PointId(pub u32);
impl From<u32> for PointId {
    fn from(id: u32) -> Self {
        PointId(id)
    }
}

struct Geometry {
    surfaces: HashMap<SurfaceId, Surface>,
    curves: HashMap<CurveId, Curve>,
    points: HashMap<PointId, Point3>,
}
