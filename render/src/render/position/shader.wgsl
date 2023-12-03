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

struct SurfaceVertexIn {
    @location(0) id: u32,
    @location(1) position: vec3<f32>,
    @location(2) tex_coords: vec2<f32>,
    @location(3) normal: vec3<f32>,
    @location(4) tangent: vec3<f32>,
    @location(5) bitangent: vec3<f32>,
    @location(6) param_coords: vec2<f32>,
};

struct SurfaceModelInstance {
    @location(7) id: u32,

    @location(8) position_matrix_0: vec4<f32>,
    @location(9) position_matrix_1: vec4<f32>,
    @location(10) position_matrix_2: vec4<f32>,
    @location(11) position_matrix_3: vec4<f32>,

    @location(12) direction_matrix_0: vec3<f32>,
    @location(13) direction_matrix_1: vec3<f32>,
    @location(14) direction_matrix_2: vec3<f32>,
}

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

struct VertexOut {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) world_position: vec3<f32>,
}

@group(0) @binding(0)
var<uniform> globals: Globals;

const LOG_DEPTH_C = 1.0;


@vertex
fn vs_surface(
    in: SurfaceVertexIn,
    model_instance: SurfaceModelInstance,
) -> VertexOut {
    var out: VertexOut;

    let model_matrix = mat4x4<f32>(
        model_instance.position_matrix_0,
        model_instance.position_matrix_1,
        model_instance.position_matrix_2,
        model_instance.position_matrix_3,
    );

    let world_position = model_matrix * vec4<f32>(in.position, 1.0);

    out.world_position = world_position.xyz;
    out.clip_position = globals.camera.view_proj * world_position;

    // Apply logarithmic depth buffer 
    let c = 1.0;
    out.clip_position.z = log(LOG_DEPTH_C * out.clip_position.z + 1.0) / log(LOG_DEPTH_C * globals.camera.zfar + 1.0) * out.clip_position.w;

    return out;
}

@vertex
fn vs_curve(
    @builtin(vertex_index) v_idx: u32,
    in: CurveVertexIn,
    model_instance: CurveModelInstance,
) -> VertexOut {
    let v_idx_i = i32(v_idx);
    let index = v_idx_i % 4;
    let is_start = index <= 1;
    let half_width = 0.5;

    var out: VertexOut;

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
    var world_pos = vec3(0.0);
    var clip_z = 0.0;
    var clip_w = 0.0;
    if is_start {
        world_pos = start_world_pos.xyz;
        final_pos = start_screen_pos;
        clip_z = start_clip_pos.z;
        clip_w = start_clip_pos.w;
    } else {
        world_pos = end_world_pos.xyz;
        final_pos = end_screen_pos;
        clip_z = end_clip_pos.z;
        clip_w = end_clip_pos.w;
    }
    final_pos += orth + travel;

    // Set the output clip position for the current point
    out.clip_position = vec4(final_pos * clip_w, clip_z, clip_w);

    out.world_position = world_pos;

    return out;
}

@vertex
fn vs_point(
    @builtin(vertex_index) v_idx: u32,
    in: PointVertexIn,
    model_instance: PointModelInstance,
) -> VertexOut {
    let v_idx_i = i32(v_idx);
    let index = v_idx_i % 4;
    let half_width = 0.5;

    var out: VertexOut;

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

    let aspect = globals.viewport_dims.x / globals.viewport_dims.y;

    let ss_pixel_size = vec2(2.0, 2.0) / globals.viewport_dims;
    let ss_half_width = ss_pixel_size * half_width;

    var u = 1.0;
    var v = 1.0;

    if index == 0 || index == 1 {
        u *= -1.0;
    }

    if index == 1 || index == 3 {
        v *= -1.0;
    }

    let uv = vec2(u, v);
    let final_pos = screen_pos.xy + ss_half_width * uv;

    // Set the output clip position for the current point
    out.clip_position = vec4(final_pos * clip_pos.w, clip_pos.z, clip_pos.w);

    out.world_position = world_pos.xyz;

    return out;
}


@fragment
fn fs_main(
    in: VertexOut
) -> @location(0) vec4<f32> {
    return vec4<f32>(
        in.world_position.x,
        in.world_position.y,
        in.world_position.z,
        1.0
    );
}