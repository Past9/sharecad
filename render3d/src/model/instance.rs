use bytemuck::{Pod, Zeroable};
use space::{Mat33, Mat44, Quat, Vec3};

use crate::render::VertexBuffer;

#[derive(Copy, Clone, Debug)]
pub struct InstanceId(pub u32);
impl From<u32> for InstanceId {
    fn from(id: u32) -> Self {
        InstanceId(id)
    }
}

pub trait Instance: std::fmt::Debug + 'static {
    type RawBuffer: VertexBuffer;

    fn id(&self) -> InstanceId;
    fn to_raw(&self) -> Self::RawBuffer;
}

#[derive(Debug, Clone)]
pub struct PositionedInstance {
    pub id: InstanceId,
    pub position: Vec3,
    pub rotation: Quat,
}
impl Instance for PositionedInstance {
    type RawBuffer = InstanceRaw;

    fn id(&self) -> InstanceId {
        self.id
    }

    fn to_raw(&self) -> Self::RawBuffer {
        let model = Mat44::translation(self.position) * Mat44::from(self.rotation);
        InstanceRaw {
            model: model.into(),
            normal: Mat33::from(self.rotation).into(),
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Pod, Zeroable)]
pub struct InstanceRaw {
    pub model: [[f32; 4]; 4],
    pub normal: [[f32; 3]; 3],
}
impl InstanceRaw {
    const ATTRIBS: [wgpu::VertexAttribute; 7] = wgpu::vertex_attr_array![
        6 => Float32x4,
        7 => Float32x4,
        8 => Float32x4,
        9 => Float32x4,

        10 => Float32x3,
        11 => Float32x3,
        12 => Float32x3,
    ];
}
impl VertexBuffer for InstanceRaw {
    fn desc() -> wgpu::VertexBufferLayout<'static> {
        use std::mem;
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<InstanceRaw>() as wgpu::BufferAddress,
            // We need to switch from using a step mode of Vertex to Instance
            // This means that our shaders will only change to use the next
            // instance when the shader starts processing a new instance
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &Self::ATTRIBS,
        }
    }
}
