@group(0) @binding(0)
var t_surface_albedo: texture_2d<f32>;
@group(0) @binding(1)
var s_surface_albedo: sampler;
@group(0) @binding(2)
var t_surface_normal: texture_2d<f32>;
@group(0) @binding(3)
var s_surface_normal: sampler;
@group(0) @binding(4)
var t_surface_emissive: texture_2d<f32>;
@group(0) @binding(5)
var s_surface_emissive: sampler;
@group(0) @binding(6)
var t_surface_roughness: texture_2d<f32>;
@group(0) @binding(7)
var s_surface_roughness: sampler;
@group(0) @binding(8)
var t_surface_metallic: texture_2d<f32>;
@group(0) @binding(9)
var s_surface_metallic: sampler;
@group(0) @binding(10)
var t_surface_ambient: texture_2d<f32>;
@group(0) @binding(11)
var s_surface_ambient: sampler;
@group(0) @binding(12)
var t_surface_transmit: texture_2d<f32>;
@group(0) @binding(13)
var s_surface_transmit: sampler;