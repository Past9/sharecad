use std::cell::OnceCell;

use bytemuck::{Pod, Zeroable};
use common::SurfaceId;
use space::{point2, vec2, Point2, Point3, Vec2, Vec3};
use tessellate::{SurfaceVert, TessellatedSurface};
use visual::material::SurfaceMaterialId;
use wgpu::util::DeviceExt;

use crate::render::VertexBuffer;

#[derive(Debug)]
pub struct SceneSurface {
    pub mesh: SurfaceMesh,
    pub material_id: Option<SurfaceMaterialId>,
}
impl SceneSurface {
    pub fn new(mesh: SurfaceMesh, material_id: Option<SurfaceMaterialId>) -> Self {
        Self { mesh, material_id }
    }

    pub fn material_id(&self) -> Option<SurfaceMaterialId> {
        self.material_id
    }

    pub fn vertex_buffer(&self, id: &SurfaceId, device: &wgpu::Device) -> &wgpu::Buffer {
        self.mesh.vertex_buffer(id, device)
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

    pub fn from_tessellated(tessellated: &TessellatedSurface) -> Self {
        Self::new(
            tessellated
                .points
                .iter()
                .map(SurfaceVertex::from_tessellator_vertex)
                .collect(),
            tessellated.indices.to_vec(),
        )
    }

    pub fn vertices(&self) -> &[SurfaceVertex] {
        &self.vertices
    }

    fn vertex_buffer(&self, id: &SurfaceId, device: &wgpu::Device) -> &wgpu::Buffer {
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
    pub fn from_tessellator_vertex(vert: &SurfaceVert) -> Self {
        Self {
            position: vert.pos,
            tex_coords: point2(vert.u, vert.v),
            normal: vert.normal,
            tangent: vert.tangents.0,
            bitangent: vert.tangents.1,
            param_coords: vec2(vert.u, vert.v),
        }
    }

    pub fn to_raw(&self, id: &SurfaceId) -> SurfaceVertexRaw {
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
