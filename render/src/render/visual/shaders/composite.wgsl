//#include "../../shader-includes/globals.wgsl"

struct ScreenVertexIn {
    @location(0) position: vec2<f32>,
    @location(1) tex_coord: vec2<f32>,
};

struct ScreenVertexOut {
    @builtin(position) position: vec4<f32>,
    @location(0) tex_coord: vec2<f32>,
};

@vertex
fn vs_composite(
    in: ScreenVertexIn,
) -> ScreenVertexOut {
    var out: ScreenVertexOut;
    out.position = vec4<f32>(in.position, 0.0, 1.0);
    out.tex_coord = in.tex_coord;
    return out;
}

@group(1) @binding(0)
var t_opaque_target: texture_multisampled_2d<f32>;
@group(1) @binding(1)
var s_opaque_target: sampler;
@group(1) @binding(2)
var t_accum_target: texture_multisampled_2d<f32>;
@group(1) @binding(3)
var s_accum_target: sampler;
@group(1) @binding(4)
var t_surface_transmit_target: texture_multisampled_2d<f32>;
@group(1) @binding(5)
var s_surface_transmit_target: sampler;

@fragment
fn fs_composite(
    in: ScreenVertexOut,
    @builtin(sample_index) sample_index: u32
) -> @location(0) vec4<f32> {
    var tc = (in.tex_coord * globals.viewport_dims);
    var tc2: vec2<i32> = vec2<i32>(i32(tc.x), i32(tc.y));
    var si = i32(sample_index);
    var color_background = textureLoad(t_opaque_target, tc2, si).rgb;
    var color_transmit = textureLoad(t_surface_transmit_target, tc2, si).r;
    var color_accum = textureLoad(t_accum_target, tc2, si).rgba;

    let avg_color = color_accum.rgb / max(color_accum.a, 0.00001);

    var accum_part = vec3(0.0);
    if color_accum.a != 0.0 {
        accum_part = color_accum.rgb / color_accum.a;
    }

    var color = accum_part * (1.0 - color_transmit) + color_background;

    return vec4(color, 1.0);
}