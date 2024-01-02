use crate::render::VertexBuffer;
use bytemuck::{Pod, Zeroable};
use common::PointId;
use space::Point3;
use std::cell::OnceCell;
use visual::material::PointMaterialId;
use wgpu::util::DeviceExt;

#[derive(Debug)]
pub struct ScenePoint {
    pub mesh: PointMesh,
    pub material_id: PointMaterialId,
    pub width: f32,
}
impl ScenePoint {
    pub fn new(position: Point3, material_id: PointMaterialId, width: f32) -> Self {
        Self {
            mesh: PointMesh::new(position),
            material_id,
            width,
        }
    }

    pub fn mesh(&self) -> &PointMesh {
        &self.mesh
    }

    pub fn material_id(&self) -> PointMaterialId {
        self.material_id
    }

    pub fn vertex_buffer(&self, id: &PointId, device: &wgpu::Device) -> &wgpu::Buffer {
        self.mesh.vertex_buffer(id, self.width, device)
    }

    pub fn index_buffer(&self, device: &wgpu::Device) -> &wgpu::Buffer {
        self.mesh.index_buffer(device)
    }

    pub fn num_elements(&self) -> u32 {
        self.mesh.num_elements()
    }
}

#[derive(Debug)]
pub struct PointMesh {
    vertices: [PointVertex; 4],
    indices: [u32; 6],
    vertex_buffer: OnceCell<wgpu::Buffer>,
    index_buffer: OnceCell<wgpu::Buffer>,
}
impl PointMesh {
    pub fn new(point: Point3) -> Self {
        Self {
            vertices: [PointVertex { position: point }; 4],
            indices: [2, 0, 1, 2, 1, 3],
            vertex_buffer: OnceCell::new(),
            index_buffer: OnceCell::new(),
        }
    }

    pub fn vertex_buffer(&self, id: &PointId, width: f32, device: &wgpu::Device) -> &wgpu::Buffer {
        self.vertex_buffer.get_or_init(|| {
            let vertex_data = self
                .vertices
                .iter()
                .map(|vertex| vertex.to_raw(id, width))
                .collect::<Vec<_>>();

            device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: None,
                contents: bytemuck::cast_slice(&vertex_data),
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

#[derive(Debug, Clone, Copy)]
pub struct PointVertex {
    pub position: Point3,
}
impl PointVertex {
    pub fn to_raw(&self, id: &PointId, width: f32) -> PointVertexRaw {
        PointVertexRaw {
            id: id.0,
            position: self.position.to_f32s(),
            width,
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct PointVertexRaw {
    pub id: u32,
    pub position: [f32; 3],
    pub width: f32,
}
impl PointVertexRaw {
    const ATTRIBS: [wgpu::VertexAttribute; 3] = wgpu::vertex_attr_array![
        0 => Uint32,
        1 => Float32x3,
        2 => Float32,
    ];
}
impl VertexBuffer for PointVertexRaw {
    fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<PointVertexRaw>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBS,
        }
    }
}
