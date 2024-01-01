//#include "includes/globals.wgsl"

struct PointVertexIn {
    @location(0) id: u32,
    @location(1) position: vec3<f32>,
    @location(2) width: f32,
}

struct PointModelInstance {
    @location(3) id: u32,

    @location(4) position_matrix_0: vec4<f32>,
    @location(5) position_matrix_1: vec4<f32>,
    @location(6) position_matrix_2: vec4<f32>,
    @location(7) position_matrix_3: vec4<f32>,

    @location(8) direction_matrix_0: vec3<f32>,
    @location(9) direction_matrix_1: vec3<f32>,
    @location(10) direction_matrix_2: vec3<f32>,
};

struct PointVertexOut {
    @builtin(position) clip_position: vec4<f32>,
    // Half the width of the point in screen space
    @location(0) ss_half_width: vec2<f32>,
    @location(1) uv: vec2<f32>,
    @location(2) width: f32,
}

@vertex
fn vs_point(
    @builtin(vertex_index) v_idx: u32,
    in: PointVertexIn,
    model_instance: PointModelInstance,
) -> PointVertexOut {
    let v_idx_i = i32(v_idx);
    let index = v_idx_i % 4;
    let half_width = globals.pixels_per_point * in.width / 2.0;

    var out: PointVertexOut;

    let position_matrix = mat4x4<f32>(
        model_instance.position_matrix_0,
        model_instance.position_matrix_1,
        model_instance.position_matrix_2,
        model_instance.position_matrix_3,
    );

    let world_pos = position_matrix * vec4(in.position, 1.0);

    // Transform the start and end points into clip space
    let clip_pos = globals.camera.view_proj * world_pos;

    // Transform the start and end points into screen space
    let screen_pos = clip_pos.xy / clip_pos.w;

    let ss_pixel_size = vec2(2.0, 2.0) / globals.viewport_dims;
    let ss_half_width = ss_pixel_size * half_width;

    var u = 1.0;
    var v = 1.0;

    if index == 0 || index == 1 {
        u = -1.0;
    }

    if index == 1 || index == 3 {
        v = -1.0;
    }

    let uv = vec2(u, v);
    let final_pos = screen_pos.xy + ss_half_width * uv;

    // Set the output clip position for the current point
    out.clip_position = vec4(final_pos * clip_pos.w, clip_pos.z, clip_pos.w);
    
    // TODO: This doesn't quite make sense. The half-width in screen space
    // is only necessarily equal to length(x_vec) if the aspect ratio is 1.0.
    // This needs to be adjusted to somehow account for both directions, or
    // allow the fragment shader to account for it.
    out.ss_half_width = ss_half_width;

    out.uv = uv;
    out.width = in.width;

    return out;
}


@group(0) @binding(0)
var t_point_color: texture_2d<f32>;
@group(0) @binding(1)
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