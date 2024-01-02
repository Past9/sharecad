mod curve;
mod material;
mod model;
mod point;
mod surface;

use common::{CurveId, PointId, SurfaceId};
pub use curve::*;
pub use material::*;
pub use model::*;
pub use point::*;
pub use surface::*;

#[derive(Debug, Copy, Clone)]
pub enum GeometryId {
    Surface(SurfaceId),
    Curve(CurveId),
    Point(PointId),
}
impl GeometryId {
    pub fn into_shader_value(&self) -> u32 {
        let (instance_id, type_id): (u32, u32) = match self {
            GeometryId::Surface(id) => (id.0, 1),
            GeometryId::Curve(id) => (id.0, 2),
            GeometryId::Point(id) => (id.0, 3),
        };

        (instance_id << 2) | type_id
    }

    pub fn from_shader_value(value: u32) -> Option<Self> {
        let type_id = value & 0b11;
        let instance_id = value >> 2;

        if instance_id > 0 {
            match type_id {
                1 => Some(SurfaceId(instance_id).into()),
                2 => Some(CurveId(instance_id).into()),
                3 => Some(PointId(instance_id).into()),
                _ => None,
            }
        } else {
            None
        }
    }

    pub fn value(&self) -> u32 {
        match self {
            GeometryId::Surface(id) => id.0,
            GeometryId::Curve(id) => id.0,
            GeometryId::Point(id) => id.0,
        }
    }
}
impl From<SurfaceId> for GeometryId {
    fn from(id: SurfaceId) -> Self {
        Self::Surface(id)
    }
}
impl From<CurveId> for GeometryId {
    fn from(id: CurveId) -> Self {
        Self::Curve(id)
    }
}
impl From<PointId> for GeometryId {
    fn from(id: PointId) -> Self {
        Self::Point(id)
    }
}
