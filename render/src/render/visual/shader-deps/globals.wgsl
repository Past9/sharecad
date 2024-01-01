struct Globals {
    @align(16) num_directional_lights: u32,
    @align(16) num_ambient_lights: u32,
    @align(16) viewport_dims: vec2<f32>,
    @align(16) pixels_per_point: f32,
    @align(16) camera: Camera
}

@group(1) @binding(0)
var<uniform> globals: Globals;