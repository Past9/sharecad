struct DirectionalLight {
    @location(0) direction: vec3<f32>,
    @location(1) color: vec3<f32>,
};

struct AmbientLight {
    @location(0) color: vec3<f32>,
};

@group(2) @binding(0) 
var<storage, read> directional_lights: array<DirectionalLight>;

@group(2) @binding(1) 
var<storage, read> ambient_lights: array<AmbientLight>;