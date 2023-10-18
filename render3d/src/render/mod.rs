mod visual;
use bytemuck::{Pod, Zeroable};

pub use visual::*;

pub trait VertexBuffer: Pod + Zeroable {
    fn desc() -> wgpu::VertexBufferLayout<'static>;
}
