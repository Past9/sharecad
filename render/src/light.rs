use bytemuck::{Pod, Zeroable};
use space::{Point3, Vec3};

use crate::color::Rgb;

#[derive(Debug, Clone)]
pub struct DirectionalLight {
    pub direction: Vec3,
    pub color: Rgb,
}
impl DirectionalLight {
    pub fn new(direction: Vec3, color: Rgb) -> Self {
        Self { direction, color }
    }

    pub fn to_raw(&self) -> DirectionalLightRaw {
        DirectionalLightRaw {
            direction: self.direction.to_f32s(),
            _padding: 0,
            color: self.color.as_f32s(),
            _padding2: 0,
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
pub struct DirectionalLightRaw {
    pub direction: [f32; 3],
    pub _padding: u32,
    pub color: [f32; 3],
    pub _padding2: u32,
}
impl Default for DirectionalLightRaw {
    fn default() -> Self {
        Self {
            direction: [0.0; 3],
            _padding: 0,
            color: [0.0; 3],
            _padding2: 0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct AmbientLight {
    pub color: Rgb,
}
impl AmbientLight {
    pub fn new(color: Rgb) -> Self {
        Self { color }
    }

    pub fn to_raw(&self) -> AmbientLightRaw {
        AmbientLightRaw {
            color: self.color.as_f32s(),
            _padding: 0,
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
pub struct AmbientLightRaw {
    color: [f32; 3],
    _padding: u32,
}
impl Default for AmbientLightRaw {
    fn default() -> Self {
        Self {
            color: [0.0; 3],
            _padding: 0,
        }
    }
}

/*
#[derive(Clone, Debug)]
pub struct DirectionalLight {
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

 */
