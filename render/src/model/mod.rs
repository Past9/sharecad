mod curve;
mod material;
mod point;
mod surface;

pub use curve::*;
pub use material::*;
pub use point::*;
pub use surface::*;

#[derive(Debug, Copy, Clone)]
pub enum InstanceId {
    Surface(SurfaceInstanceId),
    Curve(CurveInstanceId),
    Point(PointInstanceId),
}
impl InstanceId {
    pub fn into_shader_value(&self) -> u32 {
        let (instance_id, type_id): (u32, u32) = match self {
            InstanceId::Surface(id) => (id.0, 1),
            InstanceId::Curve(id) => (id.0, 2),
            InstanceId::Point(id) => (id.0, 3),
        };

        (instance_id << 2) | type_id
    }

    pub fn from_shader_value(value: u32) -> Option<Self> {
        let type_id = value & 0b11;
        let instance_id = value >> 2;

        if instance_id > 0 {
            match type_id {
                1 => Some(SurfaceInstanceId(instance_id).into()),
                2 => Some(CurveInstanceId(instance_id).into()),
                3 => Some(PointInstanceId(instance_id).into()),
                _ => None,
            }
        } else {
            None
        }
    }

    pub fn value(&self) -> u32 {
        match self {
            InstanceId::Surface(id) => id.0,
            InstanceId::Curve(id) => id.0,
            InstanceId::Point(id) => id.0,
        }
    }
}
impl From<SurfaceInstanceId> for InstanceId {
    fn from(id: SurfaceInstanceId) -> Self {
        Self::Surface(id)
    }
}
impl From<CurveInstanceId> for InstanceId {
    fn from(id: CurveInstanceId) -> Self {
        Self::Curve(id)
    }
}
impl From<PointInstanceId> for InstanceId {
    fn from(id: PointInstanceId) -> Self {
        Self::Point(id)
    }
}
