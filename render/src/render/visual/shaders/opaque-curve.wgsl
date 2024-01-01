//#include "includes/globals.wgsl"

struct CurveVertexIn {
    @location(0) id: u32,
    @location(1) position: vec3<f32>,
    @location(2) direction: vec3<f32>,
    @location(3) width: f32,
};

struct CurveModelInstance {
    @location(4) id: u32,

    @location(5) position_matrix_0: vec4<f32>,
    @location(6) position_matrix_1: vec4<f32>,
    @location(7) position_matrix_2: vec4<f32>,
    @location(8) position_matrix_3: vec4<f32>,

    @location(9) direction_matrix_0: vec3<f32>,
    @location(10) direction_matrix_1: vec3<f32>,
    @location(11) direction_matrix_2: vec3<f32>,
}

struct CurveVertexOut {
    @builtin(position) clip_position: vec4<f32>,
    // Half the width of the line in screen space
    @location(0) ss_half_width: f32,
    @location(1) tint: vec4<f32>,
}

@vertex
fn vs_curve(
    @builtin(vertex_index) v_idx: u32,
    in: CurveVertexIn,
    model_instance: CurveModelInstance,
) -> CurveVertexOut {
    let v_idx_i = i32(v_idx);
    let index = v_idx_i % 4;
    let is_start = index <= 1;
    let half_width = globals.pixels_per_point * in.width / 2.0;

    var out: CurveVertexOut;

    let position_matrix = mat4x4<f32>(
        model_instance.position_matrix_0,
        model_instance.position_matrix_1,
        model_instance.position_matrix_2,
        model_instance.position_matrix_3,
    );

    let direction_matrix = mat3x3<f32>(
        model_instance.direction_matrix_0,
        model_instance.direction_matrix_1,
        model_instance.direction_matrix_2,
    );

    // Depending on the index, the current vertex is either at the start 
    // or end of the line segment. Using this information and the line's 
    // direction vector in world space, we find the world space coordinates
    // of both the start and end points of the segment.
    var start = vec3(0.0);
    var end = vec3(0.0);
    if is_start {
        start = in.position;
        end = in.position + in.direction;
    } else {
        start = in.position - in.direction;
        end = in.position;
    }

    let start_world_pos = position_matrix * vec4(start, 1.0);
    let end_world_pos = position_matrix * vec4(end, 1.0);

    // Transform the start and end points into clip space
    let start_clip_pos = globals.camera.view_proj * start_world_pos;
    let end_clip_pos = globals.camera.view_proj * end_world_pos;

    // Transform the start and end points into screen space
    let start_screen_pos = start_clip_pos.xy / start_clip_pos.w;
    let end_screen_pos = end_clip_pos.xy / end_clip_pos.w;

    // Get the screen space direction of the line in screen space
    let ss_direction = (end_screen_pos - start_screen_pos);

    // Now that we have a screen space direction, we can expand the vertices
    // to form a camera-aligned quad of the desired width.

    let aspect = globals.viewport_dims.x / globals.viewport_dims.y;

    // Get a vector perpendicular to the line direction and flip it if needed. 
    // Make its magnitude half the desired line width.
    var flip_orth: f32 = 1.0;
    if v_idx_i % 2 == 1 {
        flip_orth = -1.0;
    }
    var orth = normalize(vec2(-ss_direction.y / aspect, ss_direction.x)) * flip_orth * half_width;
    // Scale the vector so it's in screen space coordinates instead of pixels
    orth *= 2.0 / globals.viewport_dims;


    // Get a vector along the line's direction and flip it if needed. Make its magnitude 
    // half the desired line width.
    var flip_travel: f32 = 1.0;
    if is_start {
        flip_travel = -1.0;
    }
    flip_travel *= 0.0;
    var travel = normalize(vec2(ss_direction.x / aspect, ss_direction.y)) * flip_travel * half_width;
    // Scale the vector so it's in screen space coordinates instead of pixels
    travel *= 2.0 / globals.viewport_dims;

    // Move the vertex along those vectors
    var final_pos = vec2(0.0);
    var clip_z = 0.0;
    var clip_w = 0.0;
    if is_start {
        final_pos = start_screen_pos;
        clip_z = start_clip_pos.z;
        clip_w = start_clip_pos.w;
    } else {
        final_pos = end_screen_pos;
        clip_z = end_clip_pos.z;
        clip_w = end_clip_pos.w;
    }
    final_pos += orth + travel;

    // Set the output clip position for the current point
    out.clip_position = vec4(final_pos * clip_w, clip_z, clip_w);
    
    // TODO: This doesn't quite make sense. The half-width in screen space
    // is only necessarily equal to length(orth) if the aspect ratio is 1.0.
    // This needs to be adjusted to somehow account for both directions, or
    // allow the fragment shader to account for it.
    out.ss_half_width = length(orth);

    //out.tint = model_instance.tint;

    return out;
}

@group(0) @binding(0)
var t_curve_color: texture_2d<f32>;
@group(0) @binding(1)
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