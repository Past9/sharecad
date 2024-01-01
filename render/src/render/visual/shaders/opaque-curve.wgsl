//#include "../../shader-includes/globals.wgsl"
//#include "../../shader-includes/vs-curve.wgsl"


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
    // Adjust the z-depth of the fragment so that it's closer to the camera by 
    // the half_width below:

    // Get the actual W, not the weird reciprocal that OpenGL provides for some reason.
    let w = 1.0 / in.clip_position.w;

    // Reverse-transform Z
    var z = in.clip_position.z * w;

    // Get a scaling factor that maps pixels to a depth distance
    var scale = 2.0 * globals.camera.scale.z / sqrt(pow(globals.camera.scale.x, 2.0) + pow(globals.camera.scale.y, 2.0));

    // Move the Z by `half_width` "pixels" towards the camera
    z -= in.ss_half_width * scale;

    var color: vec3<f32> = textureSample(t_curve_color, s_curve_color, vec2(0.5, 0.5)).rgb;

    // Apply tint
    color = (1.0 - in.tint.a) * color + (in.tint.rgb * in.tint.a);

    var out: FsOpaqueCurveOut;

    out.depth = z / w;
    out.color = vec4(color, 1.0);

    return out;
}