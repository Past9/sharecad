use std::cell::OnceCell;

use bytemuck::{Pod, Zeroable};
use space::{Mat33, Mat44, Point2, Point3, Quat, Vec2, Vec3};
use wgpu::util::DeviceExt;

use crate::{color::Rgba, render::VertexBuffer};

use super::SurfaceMaterialId;

#[derive(Copy, Clone, Debug)]
pub struct SurfaceId(pub u32);
impl From<u32> for SurfaceId {
    fn from(id: u32) -> Self {
        SurfaceId(id)
    }
}

#[derive(Debug)]
pub struct SceneSurface {
    pub id: SurfaceId,
    pub mesh: SurfaceMesh,
    pub material_id: SurfaceMaterialId,
}
impl SceneSurface {
    pub fn new(id: SurfaceId, mesh: SurfaceMesh, material_id: SurfaceMaterialId) -> Self {
        Self {
            id,
            mesh,
            material_id,
        }
    }

    pub fn material_id(&self) -> SurfaceMaterialId {
        self.material_id
    }

    pub fn vertex_buffer(&self, device: &wgpu::Device) -> &wgpu::Buffer {
        self.mesh.vertex_buffer(self.id, device)
    }

    pub fn index_buffer(&self, device: &wgpu::Device) -> &wgpu::Buffer {
        self.mesh.index_buffer(device)
    }

    pub fn num_elements(&self) -> u32 {
        self.mesh.num_elements()
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

    fn vertex_buffer(&self, id: SurfaceId, device: &wgpu::Device) -> &wgpu::Buffer {
        self.vertex_buffer.get_or_init(|| {
            let vertex_data = self
                .vertices
                .iter()
                .map(|vertex| vertex.to_raw(id))
                .collect::<Vec<_>>();

            device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: None,
                contents: bytemuck::cast_slice(&vertex_data),
                usage: wgpu::BufferUsages::VERTEX,
            })
        })
    }

    fn index_buffer(&self, device: &wgpu::Device) -> &wgpu::Buffer {
        self.index_buffer.get_or_init(|| {
            device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: None,
                contents: bytemuck::cast_slice(&self.indices),
                usage: wgpu::BufferUsages::INDEX,
            })
        })
    }

    fn num_elements(&self) -> u32 {
        self.indices.len() as u32
    }
}

#[derive(Debug)]
pub struct SurfaceVertex {
    pub position: Point3,
    pub tex_coords: Point2,
    pub normal: Vec3,
    pub tangent: Vec3,
    pub bitangent: Vec3,
    pub param_coords: Vec2,
}
impl SurfaceVertex {
    pub fn to_raw(&self, id: SurfaceId) -> SurfaceVertexRaw {
        SurfaceVertexRaw {
            id: id.0,
            position: self.position.to_f32s(),
            tex_coords: self.tex_coords.to_f32s(),
            normal: self.normal.to_f32s(),
            tangent: self.tangent.to_f32s(),
            bitangent: self.bitangent.to_f32s(),
            param_coords: self.param_coords.to_f32s(),
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct SurfaceVertexRaw {
    /// ID of the surface
    pub id: u32,

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
impl SurfaceVertexRaw {
    const ATTRIBS: [wgpu::VertexAttribute; 7] = wgpu::vertex_attr_array![
        0 => Uint32,
        1 => Float32x3,
        2 => Float32x2,
        3 => Float32x3,
        4 => Float32x3,
        5 => Float32x3,
        6 => Float32x2
    ];
}
impl VertexBuffer for SurfaceVertexRaw {
    fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<SurfaceVertexRaw>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBS,
        }
    }
}

/*
#[derive(Debug, Clone)]
pub struct SurfaceInstance {
    pub rotation: Quat,
    pub position: Vec3,
    pub tint: Rgba,
}
impl SurfaceInstance {
    fn to_raw(&self) -> SurfaceInstanceRaw {
        let model = Mat44::translation(self.position) * Mat44::from(self.rotation);
        SurfaceInstanceRaw {
            model: model.transpose().into(),
            normal: Mat33::from(self.rotation).transpose().into(),
            tint: self.tint.as_f32s(),
        }
    }
}
 */

/*
#[repr(C)]
#[derive(Copy, Clone, Pod, Zeroable)]
pub struct SurfaceInstanceRaw {
    pub model: [[f32; 4]; 4],
    pub normal: [[f32; 3]; 3],
    pub tint: [f32; 4],
}
impl SurfaceInstanceRaw {
    const ATTRIBS: [wgpu::VertexAttribute; 8] = wgpu::vertex_attr_array![
        6 => Float32x4,
        7 => Float32x4,
        8 => Float32x4,
        9 => Float32x4,

        10 => Float32x3,
        11 => Float32x3,
        12 => Float32x3,

        13 => Float32x4,
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
 */
