//#include "../../shader-includes/globals.wgsl"
//#include "../../shader-includes/vs-point.wgsl"

@group(1) @binding(0)
var t_point_color: texture_2d<f32>;
@group(1) @binding(1)
var s_point_color: sampler;

struct FsOpaquePointOut {
    @location(0) color: vec4<f32>,
    @builtin(frag_depth) depth: f32,
}

@fragment
fn fs_opaque_point(
    in: PointVertexOut,
) -> FsOpaquePointOut {
    // The bulk of this shader "pulls" the pixels at the center of the point
    // quads closer to the camera to give them a spherical appearance. Note
    // that this is an approximation and may give poor results if very 
    // large-radius points are used, especially at extreme aspect ratios and
    // when the point is displayed far from the center of the viewport.
    //
    // The rest of the shader discards pixels on the corners of the point 
    // quads to make them circular, then feathers the edges for some cheap
    // anti-aliasing.

    var color: vec3<f32> = textureSample(t_point_color, s_point_color, in.uv).rgb;

    if length(in.uv) > 1.0 {
        discard;
    }

    let w = 1.0 / in.clip_position.w;
    var z = in.clip_position.z * w;

    var aspect_ratio = globals.viewport_dims.x / globals.viewport_dims.y;

    // Get a scaling factor that maps pixels to a depth distance
    var scale = globals.camera.scale.z / sqrt(pow(globals.camera.scale.x, 2.0) + pow(globals.camera.scale.y, 2.0));

    scale *= sqrt(1.0 - pow(length(in.uv), 2.0));

    // Move the Z by `half_width` "pixels" towards the camera
    z -= length(in.ss_half_width) * scale;


    // Feather the edges for anti-aliasing
    let FEATHER_RADIUS: f32 = 1.0;
    let distance = length(in.uv);
    let half_width = in.width / 2.0;
    let full_alpha_radius = 1.0 - (FEATHER_RADIUS / half_width);
    let alpha = 1.0 - (distance - full_alpha_radius) / (1.0 - full_alpha_radius);


    //var color = vec3(0.0, 0.0, 0.0);

    // Apply tint
    let tint = vec4(0.0, 0.0, 0.0, 0.0);
    color = (1.0 - tint.a) * color + (tint.rgb * tint.a);

    var out: FsOpaquePointOut;

    out.depth = z / w;
    out.color = vec4(color, alpha);

    return out;
}