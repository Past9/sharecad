use std::cell::OnceCell;

use bytemuck::{Pod, Zeroable};
use space::{Mat33, Mat44, Quat, Vec3};
use wgpu::util::DeviceExt;

use crate::render::VertexBuffer;

use super::{SceneCurve, ScenePoints, SceneSurface};

#[derive(Copy, Clone, Debug)]
pub struct ModelId(pub u32);
impl From<u32> for ModelId {
    fn from(id: u32) -> Self {
        ModelId(id)
    }
}

#[derive(Debug)]
pub struct SceneModel {
    surfaces: Vec<SceneSurface>,
    curves: Vec<SceneCurve>,
    points: Vec<ScenePoints>,
    instances: Vec<ModelInstance>,
    surface_instance_buffer: OnceCell<wgpu::Buffer>,
}
impl SceneModel {
    pub fn new(
        surfaces: Vec<SceneSurface>,
        curves: Vec<SceneCurve>,
        points: Vec<ScenePoints>,
        instances: Vec<ModelInstance>,
    ) -> Self {
        Self {
            surfaces,
            curves,
            points,
            instances,
            surface_instance_buffer: OnceCell::new(),
        }
    }

    pub fn surfaces(&self) -> &[SceneSurface] {
        &self.surfaces
    }

    pub fn curves(&self) -> &[SceneCurve] {
        &self.curves
    }

    pub fn points(&self) -> &[ScenePoints] {
        &self.points
    }

    pub fn instances(&self) -> &[ModelInstance] {
        &self.instances
    }

    pub fn surface_instance_buffer(&self, device: &wgpu::Device) -> &wgpu::Buffer {
        self.surface_instance_buffer.get_or_init(|| {
            let instance_data = self
                .instances
                .iter()
                .map(|inst| inst.to_raw())
                .collect::<Vec<_>>();

            device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: None,
                contents: bytemuck::cast_slice(&instance_data),
                usage: wgpu::BufferUsages::VERTEX,
            })
        })
    }

    pub fn num_instances(&self) -> u32 {
        self.instances.len() as u32
    }
}

#[derive(Debug, Clone)]
pub struct ModelInstance {
    pub id: ModelId,
    pub rotation: Quat,
    pub position: Vec3,
}
impl ModelInstance {
    fn to_raw(&self) -> ModelInstanceRaw {
        let model = Mat44::translation(self.position) * Mat44::from(self.rotation);
        ModelInstanceRaw {
            id: self.id.0,
            model: model.transpose().into(),
            normal: Mat33::from(self.rotation).transpose().into(),
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Pod, Zeroable)]
pub struct ModelInstanceRaw {
    pub id: u32,
    pub model: [[f32; 4]; 4],
    pub normal: [[f32; 3]; 3],
}
impl ModelInstanceRaw {
    const SURFACE_ATTRIBS: [wgpu::VertexAttribute; 8] = wgpu::vertex_attr_array![
        7 => Uint32,

        8 => Float32x4,
        9 => Float32x4,
        10 => Float32x4,
        11 => Float32x4,

        12 => Float32x3,
        13 => Float32x3,
        14=> Float32x3,
    ];

    pub fn surface_desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<ModelInstanceRaw>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &Self::SURFACE_ATTRIBS,
        }
    }
}
