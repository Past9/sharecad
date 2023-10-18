mod instance;
mod mesh;

use crate::{material::MaterialId, render::VertexBuffer};

pub use instance::*;
pub use mesh::*;

pub trait SceneObject: std::fmt::Debug {
    fn mesh(&self) -> &Mesh;
    fn instance_buffer(&self, device: &wgpu::Device) -> &wgpu::Buffer;
    fn material_id(&self) -> MaterialId;
    fn num_instances(&self) -> u32;
}
