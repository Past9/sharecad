use std::cell::OnceCell;

use bytemuck::{Pod, Zeroable};
use space::{Mat44, Quat, Vec3};
use wgpu::util::DeviceExt;

use crate::render::VertexBuffer;

use super::CurveMaterialId;

pub trait SceneCurve: std::fmt::Debug {
    fn mesh(&self) -> &CurveMesh;
    fn instance_buffer(&self, device: &wgpu::Device) -> &wgpu::Buffer;
    fn material_id(&self) -> CurveMaterialId;
    fn num_instances(&self) -> u32;
}

#[derive(Debug)]
pub struct SceneCurveObject<T: SceneCurveInstance> {
    pub mesh: CurveMesh,
    pub instances: Vec<T>,
    pub material_id: CurveMaterialId,
    instance_buffer: OnceCell<wgpu::Buffer>,
}
impl<T: SceneCurveInstance> SceneCurveObject<T> {
    pub fn new(mesh: CurveMesh, instances: Vec<T>, material_id: CurveMaterialId) -> Self {
        Self {
            mesh,
            instances,
            material_id,
            instance_buffer: OnceCell::new(),
        }
    }
}
impl<T: SceneCurveInstance> SceneCurve for SceneCurveObject<T> {
    fn mesh(&self) -> &CurveMesh {
        &self.mesh
    }

    fn instance_buffer(&self, device: &wgpu::Device) -> &wgpu::Buffer {
        self.instance_buffer.get_or_init(|| {
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

    fn material_id(&self) -> CurveMaterialId {
        self.material_id
    }

    fn num_instances(&self) -> u32 {
        self.instances.len() as u32
    }
}

#[derive(Debug)]
pub struct CurveMesh {
    vertices: Vec<CurveVertex>,
    indices: Vec<u32>,
    vertex_buffer: OnceCell<wgpu::Buffer>,
    index_buffer: OnceCell<wgpu::Buffer>,
}
impl CurveMesh {
    pub fn new(vertices: Vec<CurveVertex>, indices: Vec<u32>) -> Self {
        Self {
            vertices,
            indices,
            vertex_buffer: OnceCell::new(),
            index_buffer: OnceCell::new(),
        }
    }

    pub fn vertex_buffer(&self, device: &wgpu::Device) -> &wgpu::Buffer {
        self.vertex_buffer.get_or_init(|| {
            device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: None,
                contents: bytemuck::cast_slice(&self.vertices),
                usage: wgpu::BufferUsages::VERTEX,
            })
        })
    }

    pub fn index_buffer(&self, device: &wgpu::Device) -> &wgpu::Buffer {
        self.index_buffer.get_or_init(|| {
            device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: None,
                contents: bytemuck::cast_slice(&self.indices),
                usage: wgpu::BufferUsages::INDEX,
            })
        })
    }

    pub fn num_elements(&self) -> u32 {
        self.indices.len() as u32
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct CurveVertex {
    pub position: [f32; 3],
    pub width: f32,
}
impl CurveVertex {
    const ATTRIBS: [wgpu::VertexAttribute; 2] = wgpu::vertex_attr_array![
        0 => Float32x3,
        1 => Float32
    ];
}
impl VertexBuffer for CurveVertex {
    fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<CurveVertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBS,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct CurveInstanceId(pub u32);
impl From<u32> for CurveInstanceId {
    fn from(id: u32) -> Self {
        CurveInstanceId(id)
    }
}

pub trait SceneCurveInstance: std::fmt::Debug + Clone + 'static {
    type RawBuffer: VertexBuffer;

    fn id(&self) -> CurveInstanceId;
    fn to_raw(&self) -> Self::RawBuffer;
}

#[derive(Debug, Clone)]
pub struct TransformedCurveInstance {
    pub id: CurveInstanceId,
    pub scale: Vec3,
    pub rotation: Quat,
    pub position: Vec3,
}
impl SceneCurveInstance for TransformedCurveInstance {
    type RawBuffer = TransformedCurveInstanceRaw;

    fn id(&self) -> CurveInstanceId {
        self.id
    }

    fn to_raw(&self) -> Self::RawBuffer {
        let model = Mat44::translation(self.position)
            * Mat44::from(self.rotation)
            * Mat44::scale(self.scale);
        TransformedCurveInstanceRaw {
            model: model.transpose().into(),
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Pod, Zeroable)]
pub struct TransformedCurveInstanceRaw {
    pub model: [[f32; 4]; 4],
}
impl TransformedCurveInstanceRaw {
    const ATTRIBS: [wgpu::VertexAttribute; 4] = wgpu::vertex_attr_array![
        6 => Float32x4,
        7 => Float32x4,
        8 => Float32x4,
        9 => Float32x4,
    ];
}
impl VertexBuffer for TransformedCurveInstanceRaw {
    fn desc() -> wgpu::VertexBufferLayout<'static> {
        use std::mem;
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<TransformedCurveInstanceRaw>() as wgpu::BufferAddress,
            // We need to switch from using a step mode of Vertex to Instance
            // This means that our shaders will only change to use the next
            // instance when the shader starts processing a new instance
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &Self::ATTRIBS,
        }
    }
}
