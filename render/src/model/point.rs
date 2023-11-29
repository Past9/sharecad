use bytemuck::{Pod, Zeroable};
use space::{Mat44, Point3, Quat, Vec3};
use std::cell::OnceCell;
use wgpu::util::DeviceExt;

use crate::{color::Rgba, render::VertexBuffer};

use super::PointMaterialId;

pub trait ScenePoint: std::fmt::Debug {
    fn mesh(&self) -> &PointMesh;
    fn instance_buffer(&self, device: &wgpu::Device) -> &wgpu::Buffer;
    fn material_id(&self) -> PointMaterialId;
    fn num_instances(&self) -> u32;
}

pub struct PointPoint {
    pub position: Point3,
    pub width: f32,
}

#[derive(Debug)]
pub struct ScenePointObject<T: ScenePointInstance> {
    pub mesh: PointMesh,
    pub instances: Vec<T>,
    pub material_id: PointMaterialId,
    instance_buffer: OnceCell<wgpu::Buffer>,
}
impl<T: ScenePointInstance> ScenePointObject<T> {
    pub fn new(mesh: PointMesh, instances: Vec<T>, material_id: PointMaterialId) -> Self {
        Self {
            mesh,
            instances,
            material_id,
            instance_buffer: OnceCell::new(),
        }
    }
}
impl<T: ScenePointInstance> ScenePoint for ScenePointObject<T> {
    fn mesh(&self) -> &PointMesh {
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

    fn material_id(&self) -> PointMaterialId {
        self.material_id
    }

    fn num_instances(&self) -> u32 {
        self.instances.len() as u32
    }
}

#[derive(Debug)]
pub struct PointMesh {
    vertices: Vec<PointVertex>,
    indices: Vec<u32>,
    vertex_buffer: OnceCell<wgpu::Buffer>,
    index_buffer: OnceCell<wgpu::Buffer>,
}
impl PointMesh {
    pub fn new(points: Vec<PointPoint>) -> Self {
        let mut vertices = Vec::with_capacity((points.len() * 4));

        for i in 0..points.len() {
            let p = &points[i];
            vertices.extend(
                [PointVertex {
                    position: p.position.to_f32s(),
                    width: p.width,
                }; 4],
            );
        }

        let indices = (0..points.len())
            .flat_map(|i| {
                let i = i as u32 * 4;
                [
                    // First triangle
                    i + 2,
                    i,
                    i + 1,
                    // Second triangle
                    i + 2,
                    i + 1,
                    i + 3,
                ]
            })
            .collect::<Vec<_>>();

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
pub struct PointVertex {
    pub position: [f32; 3],
    pub width: f32,
}
impl PointVertex {
    const ATTRIBS: [wgpu::VertexAttribute; 2] = wgpu::vertex_attr_array![
        0 => Float32x3,
        1 => Float32
    ];
}
impl VertexBuffer for PointVertex {
    fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<PointVertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBS,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct PointInstanceId(pub u32);
impl From<u32> for PointInstanceId {
    fn from(id: u32) -> Self {
        PointInstanceId(id)
    }
}

pub trait ScenePointInstance: std::fmt::Debug + Clone + 'static {
    type RawBuffer: VertexBuffer;

    fn id(&self) -> PointInstanceId;
    fn to_raw(&self) -> Self::RawBuffer;
}

#[derive(Debug, Clone)]
pub struct PointInstance {
    pub id: PointInstanceId,
    pub scale: Vec3,
    pub rotation: Quat,
    pub position: Vec3,
    pub tint: Rgba,
}
impl ScenePointInstance for PointInstance {
    type RawBuffer = PointInstanceRaw;

    fn id(&self) -> PointInstanceId {
        self.id
    }

    fn to_raw(&self) -> Self::RawBuffer {
        let position = Mat44::translation(self.position)
            * Mat44::from(self.rotation)
            * Mat44::scale(self.scale);
        PointInstanceRaw {
            position: position.transpose().into(),
            tint: self.tint.as_f32s(),
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Pod, Zeroable)]
pub struct PointInstanceRaw {
    pub position: [[f32; 4]; 4],
    pub tint: [f32; 4],
}
impl PointInstanceRaw {
    const ATTRIBS: [wgpu::VertexAttribute; 5] = wgpu::vertex_attr_array![
        2 => Float32x4,
        3 => Float32x4,
        4 => Float32x4,
        5 => Float32x4,
        6 => Float32x4,
    ];
}
impl VertexBuffer for PointInstanceRaw {
    fn desc() -> wgpu::VertexBufferLayout<'static> {
        use std::mem;
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<PointInstanceRaw>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &Self::ATTRIBS,
        }
    }
}
