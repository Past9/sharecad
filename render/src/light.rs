use bytemuck::{Pod, Zeroable};
use space::Point3;

#[derive(Clone, Debug)]
pub struct Light {
    pub position: Point3,
    pub color: [f32; 3],
}
impl Light {
    pub fn new(position: Point3, color: [f32; 3]) -> Self {
        Self { position, color }
    }

    pub fn to_raw(&self) -> LightUniform {
        LightUniform {
            position: self.position.to_f32s(),
            _padding: 0,
            color: self.color,
            _padding2: 0,
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
pub struct LightUniform {
    pub position: [f32; 3],
    pub _padding: u32,
    pub color: [f32; 3],
    pub _padding2: u32,
}
impl Default for LightUniform {
    fn default() -> Self {
        Self {
            position: Default::default(),
            _padding: Default::default(),
            color: Default::default(),
            _padding2: Default::default(),
        }
    }
}
