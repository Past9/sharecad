use std::cell::OnceCell;

use bytemuck::{Pod, Zeroable};
use space::{Mat33, Mat44, Quat, Vec3};
use wgpu::util::DeviceExt;

use crate::{color::Rgba, render::VertexBuffer};

use super::SurfaceMaterialId;

pub trait SceneSurface: std::fmt::Debug {
    fn mesh(&self) -> &SurfaceMesh;
    fn instance_buffer(&self, device: &wgpu::Device) -> &wgpu::Buffer;
    fn material_id(&self) -> SurfaceMaterialId;
    fn num_instances(&self) -> u32;
}

#[derive(Debug)]
pub struct PolySurface<T: SceneSurfaceInstance> {
    pub mesh: SurfaceMesh,
    pub instances: Vec<T>,
    pub material_id: SurfaceMaterialId,
    instance_buffer: OnceCell<wgpu::Buffer>,
}
impl<T: SceneSurfaceInstance> PolySurface<T> {
    pub fn new(mesh: SurfaceMesh, instances: Vec<T>, material_id: SurfaceMaterialId) -> Self {
        Self {
            mesh,
            instances,
            material_id,
            instance_buffer: OnceCell::new(),
        }
    }
}
impl<T: SceneSurfaceInstance> SceneSurface for PolySurface<T> {
    fn mesh(&self) -> &SurfaceMesh {
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

    fn material_id(&self) -> SurfaceMaterialId {
        self.material_id
    }

    fn num_instances(&self) -> u32 {
        self.instances.len() as u32
    }
}

#[derive(Debug)]
pub struct SurfaceMesh {
    vertices: Vec<SurfaceVertex>,
    indices: Vec<u32>,
    vertex_buffer: OnceCell<wgpu::Buffer>,
    index_buffer: OnceCell<wgpu::Buffer>,
}
impl SurfaceMesh {
    pub fn new(vertices: Vec<SurfaceVertex>, indices: Vec<u32>) -> Self {
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
pub struct SurfaceVertex {
    /// Position in world space
    pub position: [f32; 3],

    /// Texture UV coordinates
    pub tex_coords: [f32; 2],

    /// Normal vector
    pub normal: [f32; 3],

    /// Tangent vector
    pub tangent: [f32; 3],

    /// Bitangent vector
    pub bitangent: [f32; 3],

    /// Parameteric surface UV coordinates
    pub param_coords: [f32; 2],
}
impl SurfaceVertex {
    const ATTRIBS: [wgpu::VertexAttribute; 6] = wgpu::vertex_attr_array![
        0 => Float32x3,
        1 => Float32x2,
        2 => Float32x3,
        3 => Float32x3,
        4 => Float32x3,
        5 => Float32x2
    ];
}
impl VertexBuffer for SurfaceVertex {
    fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<SurfaceVertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBS,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct SurfaceInstanceId(pub u32);
impl From<u32> for SurfaceInstanceId {
    fn from(id: u32) -> Self {
        SurfaceInstanceId(id)
    }
}

pub trait SceneSurfaceInstance: std::fmt::Debug + Clone + 'static {
    type RawBuffer: VertexBuffer;

    fn id(&self) -> SurfaceInstanceId;
    fn to_raw(&self) -> Self::RawBuffer;
}

#[derive(Debug, Clone)]
pub struct SurfaceInstance {
    pub id: SurfaceInstanceId,
    pub scale: Vec3,
    pub rotation: Quat,
    pub position: Vec3,
    pub tint: Rgba,
}
impl SceneSurfaceInstance for SurfaceInstance {
    type RawBuffer = SurfaceInstanceRaw;

    fn id(&self) -> SurfaceInstanceId {
        self.id
    }

    fn to_raw(&self) -> Self::RawBuffer {
        let model = Mat44::translation(self.position)
            * Mat44::from(self.rotation)
            * Mat44::scale(self.scale);
        println!("id = {:?}", self.id);
        SurfaceInstanceRaw {
            model: model.transpose().into(),
            normal: Mat33::from(self.rotation).transpose().into(),
            tint: self.tint.as_f32s(),
            id: self.id.0,
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Pod, Zeroable)]
pub struct SurfaceInstanceRaw {
    pub model: [[f32; 4]; 4],
    pub normal: [[f32; 3]; 3],
    pub tint: [f32; 4],
    pub id: u32,
}
impl SurfaceInstanceRaw {
    const ATTRIBS: [wgpu::VertexAttribute; 9] = wgpu::vertex_attr_array![
        6 => Float32x4,
        7 => Float32x4,
        8 => Float32x4,
        9 => Float32x4,

        10 => Float32x3,
        11 => Float32x3,
        12 => Float32x3,

        13 => Float32x4,
        14 => Uint32
    ];
}
impl VertexBuffer for SurfaceInstanceRaw {
    fn desc() -> wgpu::VertexBufferLayout<'static> {
        use std::mem;
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<SurfaceInstanceRaw>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &Self::ATTRIBS,
        }
    }
}
