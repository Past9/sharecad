use std::cell::OnceCell;

use bytemuck::{Pod, Zeroable};
use space::{Mat33, Mat44, Point3, Quat, Vec3};
use wgpu::util::DeviceExt;

use crate::{color::Rgba, render::VertexBuffer};

use super::CurveMaterialId;

pub trait SceneCurve: std::fmt::Debug {
    fn mesh(&self) -> &CurveMesh;
    fn instance_buffer(&self, device: &wgpu::Device) -> &wgpu::Buffer;
    fn material_id(&self) -> CurveMaterialId;
    fn num_instances(&self) -> u32;
}

pub struct CurvePoint {
    pub position: Point3,
    pub width: f32,
}

#[derive(Debug)]
pub struct PolyCurve<T: SceneCurveInstance> {
    pub mesh: CurveMesh,
    pub instances: Vec<T>,
    pub material_id: CurveMaterialId,
    instance_buffer: OnceCell<wgpu::Buffer>,
}
impl<T: SceneCurveInstance> PolyCurve<T> {
    pub fn new(mesh: CurveMesh, instances: Vec<T>, material_id: CurveMaterialId) -> Self {
        Self {
            mesh,
            instances,
            material_id,
            instance_buffer: OnceCell::new(),
        }
    }
}
impl<T: SceneCurveInstance> SceneCurve for PolyCurve<T> {
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
    pub fn new(points: Vec<CurvePoint>) -> Self {
        let mut vertices = Vec::with_capacity((points.len() - 1) * 4);

        for i in 1..points.len() {
            let p0 = &points[i - 1];
            let p1 = &points[i];
            let line_dir = (p1.position - p0.position).to_f32s();
            let p0_pos = p0.position.to_f32s();
            let p1_pos = p1.position.to_f32s();
            vertices.extend([
                CurveVertex {
                    position: p0_pos,
                    direction: line_dir,
                    width: p0.width,
                },
                CurveVertex {
                    position: p0_pos,
                    direction: line_dir,
                    width: p0.width,
                },
                CurveVertex {
                    position: p1_pos,
                    direction: line_dir,
                    width: p1.width,
                },
                CurveVertex {
                    position: p1_pos,
                    direction: line_dir,
                    width: p1.width,
                },
            ]);
        }

        let indices = (1..points.len())
            .flat_map(|i| {
                let i = (i as u32 - 1) * 4;
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
pub struct CurveVertex {
    pub position: [f32; 3],
    pub direction: [f32; 3],
    pub width: f32,
}
impl CurveVertex {
    const ATTRIBS: [wgpu::VertexAttribute; 3] = wgpu::vertex_attr_array![
        0 => Float32x3,
        1 => Float32x3,
        2 => Float32
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
pub struct CurveInstance {
    pub id: CurveInstanceId,
    pub scale: Vec3,
    pub rotation: Quat,
    pub position: Vec3,
    pub tint: Rgba,
}
impl SceneCurveInstance for CurveInstance {
    type RawBuffer = CurveInstanceRaw;

    fn id(&self) -> CurveInstanceId {
        self.id
    }

    fn to_raw(&self) -> Self::RawBuffer {
        let position = Mat44::translation(self.position)
            * Mat44::from(self.rotation)
            * Mat44::scale(self.scale);
        CurveInstanceRaw {
            position: position.transpose().into(),
            direction: Mat33::from(self.rotation).transpose().into(),
            tint: self.tint.as_f32s(),
            id: self.id.0,
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Pod, Zeroable)]
pub struct CurveInstanceRaw {
    pub position: [[f32; 4]; 4],
    pub direction: [[f32; 3]; 3],
    pub tint: [f32; 4],
    pub id: u32,
}
impl CurveInstanceRaw {
    const ATTRIBS: [wgpu::VertexAttribute; 9] = wgpu::vertex_attr_array![
        3 => Float32x4,
        4 => Float32x4,
        5 => Float32x4,
        6 => Float32x4,

        7 => Float32x3,
        8 => Float32x3,
        9 => Float32x3,

        10 => Float32x4,

        11 => Uint32,
    ];
}
impl VertexBuffer for CurveInstanceRaw {
    fn desc() -> wgpu::VertexBufferLayout<'static> {
        use std::mem;
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<CurveInstanceRaw>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &Self::ATTRIBS,
        }
    }
}
