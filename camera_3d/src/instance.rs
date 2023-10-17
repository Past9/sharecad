use bytemuck::{Pod, Zeroable};

use crate::{
    model::Vertex,
    scene::{Instance, InstanceId},
};

pub trait VertexBuffer: Pod + Zeroable {
    fn desc() -> wgpu::VertexBufferLayout<'static>;
}

#[derive(Debug, Clone)]
pub struct CubeInstance {
    pub id: InstanceId,
    pub position: cgmath::Vector3<f32>,
    pub rotation: cgmath::Quaternion<f32>,
}
/*
impl CubeInstance {
    pub fn to_raw(&self) -> InstanceRaw {
        let model =
            cgmath::Matrix4::from_translation(self.position) * cgmath::Matrix4::from(self.rotation);
        InstanceRaw {
            model: model.into(),
            normal: cgmath::Matrix3::from(self.rotation).into(),
        }
    }
}
*/
impl Instance for CubeInstance {
    type RawBuffer = InstanceRaw;

    fn id(&self) -> crate::scene::InstanceId {
        self.id
    }

    fn to_raw(&self) -> Self::RawBuffer {
        let model =
            cgmath::Matrix4::from_translation(self.position) * cgmath::Matrix4::from(self.rotation);
        InstanceRaw {
            model: model.into(),
            normal: cgmath::Matrix3::from(self.rotation).into(),
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
