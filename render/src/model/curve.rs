use super::CurveMaterialId;
use crate::render::VertexBuffer;
use bytemuck::{Pod, Zeroable};
use common::CurveId;
use space::{Point3, Vec3};
use std::cell::OnceCell;
use wgpu::util::DeviceExt;

#[derive(Debug)]
pub struct SceneCurve {
    pub mesh: CurveMesh,
    pub material_id: CurveMaterialId,
    pub width: f32,
}
impl SceneCurve {
    pub fn new(mesh: CurveMesh, material_id: CurveMaterialId, width: f32) -> Self {
        Self {
            mesh,
            material_id,
            width,
        }
    }

    pub fn mesh(&self) -> &CurveMesh {
        &self.mesh
    }

    pub fn material_id(&self) -> CurveMaterialId {
        self.material_id
    }

    pub fn vertex_buffer(&self, id: &CurveId, device: &wgpu::Device) -> &wgpu::Buffer {
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
pub struct CurveMesh {
    vertices: Vec<CurveVertex>,
    indices: Vec<u32>,
    vertex_buffer: OnceCell<wgpu::Buffer>,
    index_buffer: OnceCell<wgpu::Buffer>,
}
impl CurveMesh {
    pub fn new(points: Vec<Point3>) -> Self {
        let mut vertices = Vec::with_capacity((points.len() - 1) * 4);

        for i in 1..points.len() {
            let p0 = points[i - 1];
            let p1 = points[i];
            let line_dir = p1 - p0;
            vertices.extend([
                CurveVertex {
                    position: p0,
                    direction: line_dir,
                },
                CurveVertex {
                    position: p0,
                    direction: line_dir,
                },
                CurveVertex {
                    position: p1,
                    direction: line_dir,
                },
                CurveVertex {
                    position: p1,
                    direction: line_dir,
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

    fn vertex_buffer(&self, id: &CurveId, width: f32, device: &wgpu::Device) -> &wgpu::Buffer {
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

#[derive(Debug)]
pub struct CurveVertex {
    pub position: Point3,
    pub direction: Vec3,
}
impl CurveVertex {
    pub fn to_raw(&self, id: &CurveId, width: f32) -> CurveVertexRaw {
        CurveVertexRaw {
            id: id.0,
            position: self.position.to_f32s(),
            direction: self.direction.to_f32s(),
            width,
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct CurveVertexRaw {
    pub id: u32,
    pub position: [f32; 3],
    pub direction: [f32; 3],
    pub width: f32,
}
impl CurveVertexRaw {
    const ATTRIBS: [wgpu::VertexAttribute; 4] = wgpu::vertex_attr_array![
        0 => Uint32,
        1 => Float32x3,
        2 => Float32x3,
        3 => Float32
    ];
}
impl VertexBuffer for CurveVertexRaw {
    fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<CurveVertexRaw>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBS,
        }
    }
}
