use std::cell::OnceCell;

use bytemuck::{Pod, Zeroable};
use wgpu::util::DeviceExt;

use crate::{material::MaterialId, render::VertexBuffer};

use super::{SceneObject, SceneObjectInstance};

#[derive(Debug)]
pub struct MeshObject<T: SceneObjectInstance> {
    pub mesh: Mesh,
    pub instances: Vec<T>,
    pub material_id: MaterialId,
    instance_buffer: OnceCell<wgpu::Buffer>,
}
impl<T: SceneObjectInstance> MeshObject<T> {
    pub fn new(mesh: Mesh, instances: Vec<T>, material_id: MaterialId) -> Self {
        Self {
            mesh,
            instances,
            material_id,
            instance_buffer: OnceCell::new(),
        }
    }
}
impl<T: SceneObjectInstance> SceneObject for MeshObject<T> {
    fn mesh(&self) -> &Mesh {
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
                label: Some("Instance buffer"),
                contents: bytemuck::cast_slice(&instance_data),
                usage: wgpu::BufferUsages::VERTEX,
            })
        })
    }

    fn material_id(&self) -> MaterialId {
        self.material_id
    }

    fn num_instances(&self) -> u32 {
        self.instances.len() as u32
    }
}

#[derive(Debug)]
pub struct Mesh {
    name: String,
    vertices: Vec<MeshVertex>,
    indices: Vec<u32>,
    vertex_buffer: OnceCell<wgpu::Buffer>,
    index_buffer: OnceCell<wgpu::Buffer>,
}
impl Mesh {
    pub fn new(name: &str, vertices: Vec<MeshVertex>, indices: Vec<u32>) -> Self {
        Self {
            name: name.into(),
            vertices,
            indices,
            vertex_buffer: OnceCell::new(),
            index_buffer: OnceCell::new(),
        }
    }

    pub fn vertex_buffer(&self, device: &wgpu::Device) -> &wgpu::Buffer {
        self.vertex_buffer.get_or_init(|| {
            device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some(&format!("{} vertex buffer", self.name)),
                contents: bytemuck::cast_slice(&self.vertices),
                usage: wgpu::BufferUsages::VERTEX,
            })
        })
    }

    pub fn index_buffer(&self, device: &wgpu::Device) -> &wgpu::Buffer {
        self.index_buffer.get_or_init(|| {
            device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some(&format!("{} index buffer", self.name)),
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
pub struct MeshVertex {
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
impl MeshVertex {
    const ATTRIBS: [wgpu::VertexAttribute; 6] = wgpu::vertex_attr_array![
        0 => Float32x3,
        1 => Float32x2,
        2 => Float32x3,
        3 => Float32x3,
        4 => Float32x3,
        5 => Float32x2
    ];
}
impl VertexBuffer for MeshVertex {
    fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<MeshVertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBS,
        }
    }
}
