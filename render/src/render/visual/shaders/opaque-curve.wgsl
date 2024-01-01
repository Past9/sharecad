//#include "../../shader-includes/globals.wgsl"
//#include "../../shader-includes/vs-curve.wgsl"
//#include "../../shader-includes/curve-adjust-depth.wgsl"


@group(1) @binding(0)
var t_curve_color: texture_2d<f32>;
@group(1) @binding(1)
var s_curve_color: sampler;

struct FsOpaqueCurveOut {
    @location(0) color: vec4<f32>,
    @builtin(frag_depth) depth: f32,
}

@fragment
fn fs_opaque_curve(
    in: CurveVertexOut,
) -> FsOpaqueCurveOut {
    // Get color and apply tint
    var color: vec3<f32> = textureSample(t_curve_color, s_curve_color, vec2(0.5, 0.5)).rgb;
    color = (1.0 - in.tint.a) * color + (in.tint.rgb * in.tint.a);

    var out: FsOpaqueCurveOut;

    out.color = vec4(color, 1.0);
    out.depth = curve_adjust_depth(in);

    return out;
}