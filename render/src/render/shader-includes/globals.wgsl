struct Globals {
    @align(16) num_directional_lights: u32,
    @align(16) num_ambient_lights: u32,
    @align(16) viewport_dims: vec2<f32>,
    @align(16) pixels_per_point: f32,
    @align(16) camera: Camera
}

struct Camera {
    view_pos: vec4<f32>,
    view_proj: mat4x4<f32>,
    zfar: f32,
    scale: vec3<f32>,
};

@group(0) @binding(0)
var<uniform> globals: Globals;