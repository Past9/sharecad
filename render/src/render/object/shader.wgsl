struct Globals {
    @align(16) num_directional_lights: u32,
    @align(16) num_ambient_lights: u32,
    @align(16) viewport_dims: vec2<f32>,
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

struct SurfaceVertexOut {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) surface_id: u32,
    @location(1) model_id: u32,
};

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
    @location(1) curve_id: u32,
    @location(2) model_id: u32,
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

struct PointVertexOut {
    @builtin(position) clip_position: vec4<f32>,
    // Half the width of the point in screen space
    @location(0) ss_half_width: vec2<f32>,
    @location(1) ss_pixel_size: vec2<f32>,
    @location(2) uv: vec2<f32>,
    @location(3) width: f32,
    @location(4) point_id: u32,
    @location(5) model_id: u32,
}

@group(0) @binding(0)
var<uniform> globals: Globals;

const LOG_DEPTH_C = 1.0;

@vertex
fn vs_surface(
    in: SurfaceVertexIn,
    model_instance: SurfaceModelInstance,
) -> SurfaceVertexOut {
    var out: SurfaceVertexOut;

    let model_matrix = mat4x4<f32>(
        model_instance.position_matrix_0,
        model_instance.position_matrix_1,
        model_instance.position_matrix_2,
        model_instance.position_matrix_3,
    );

    let world_position = model_matrix * vec4<f32>(in.position, 1.0);

    out.clip_position = globals.camera.view_proj * world_position;

    // Apply logarithmic depth buffer 
    out.clip_position.z = log(LOG_DEPTH_C * out.clip_position.z + 1.0) / log(LOG_DEPTH_C * globals.camera.zfar + 1.0) * out.clip_position.w;

    out.surface_id = in.id;
    out.model_id = model_instance.id;

    return out;
}

const CURVE_WIDTH: f32 = 20.0;

@vertex
fn vs_curve(
    @builtin(vertex_index) v_idx: u32,
    in: CurveVertexIn,
    model_instance: CurveModelInstance,
) -> CurveVertexOut {
    let v_idx_i = i32(v_idx);
    let index = v_idx_i % 4;
    let is_start = index <= 1;

    // We override the visual curve width with CURVE_WIDTH so that the 
    // mouse pointer doesn't have to be exactly on top of it, which would 
    // be difficult for users since curves are only a few pixels wide.
    let half_width = CURVE_WIDTH / 2.0;

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
    
    // TODO: This doesn't quite make sense. The half-width in screen space
    // is only necessarily equal to length(orth) if the aspect ratio is 1.0.
    // This needs to be adjusted to somehow account for both directions, or
    // allow the fragment shader to account for it.
    out.ss_half_width = length(orth);

    out.curve_id = in.id;
    out.model_id = model_instance.id;

    return out;
}

const POINT_WIDTH: f32 = 30.0; 

@vertex
fn vs_point(
    @builtin(vertex_index) v_idx: u32,
    in: PointVertexIn,
    model_instance: PointModelInstance,
) -> PointVertexOut {
    let v_idx_i = i32(v_idx);
    let index = v_idx_i % 4;
    let half_width = POINT_WIDTH / 2.0;

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
    
    // TODO: This doesn't quite make sense. The half-width in screen space
    // is only necessarily equal to length(x_vec) if the aspect ratio is 1.0.
    // This needs to be adjusted to somehow account for both directions, or
    // allow the fragment shader to account for it.

    out.ss_half_width = ss_half_width;
    out.ss_pixel_size = ss_pixel_size;
    out.uv = uv;
    out.width = in.width;

    out.point_id = in.id;
    out.model_id = model_instance.id;

    return out;
}


@fragment
fn fs_surface(
    in: SurfaceVertexOut
) -> @location(0) u32 {
    // Bitshift the ID left two places, then add the type identifier for surfaces (0b01)
    return (in.surface_id << u32(2)) | u32(1);
}

struct FsCurveOut {
    @location(0) id: u32,
    @builtin(frag_depth) depth: f32,
}

@fragment
fn fs_curve(
    in: CurveVertexOut,
) -> FsCurveOut {
    // Adjust the z-depth of the fragment so that it's closer to the camera by 
    // the half_width below:

    // Get the actual W, not the weird reciprocal that OpenGL provides for some reason.
    let w = 1.0 / in.clip_position.w;

    // Reverse-transform Z
    var z = in.clip_position.z * w;

    // Get a scaling factor that maps pixels to a depth distance
    var scale = 2.0 * globals.camera.scale.z / sqrt(pow(globals.camera.scale.x, 2.0) + pow(globals.camera.scale.y, 2.0));

    // Move the Z by `half_width` "pixels" towards the camera
    z -= in.ss_half_width * scale * w;

    // Apply logarithmic depth buffer 
    z = log(LOG_DEPTH_C * z + 1.0) / log(LOG_DEPTH_C * globals.camera.zfar + 1.0) * w;

    var out: FsCurveOut;

    out.depth = z / w;

    // Bitshift the ID left two places, then add the type identifier for surfaces (0b10)
    out.id = (in.curve_id << u32(2)) | u32(2);

    return out;
}

struct FsPointOut {
    @location(0) id: u32,
    @builtin(frag_depth) depth: f32,
}

@fragment
fn fs_point(
    in: PointVertexOut,
) -> FsPointOut {
    // The bulk of this shader "pulls" the pixels at the center of the point
    // quads closer to the camera to give them a spherical appearance. Note
    // that this is an approximation and may give poor results if very 
    // large-radius points are used, especially at extreme aspect ratios and
    // when the point is displayed far from the center of the viewport.
    //
    // The rest of the shader discards pixels on the corners of the point 
    // quads to make them circular, then feathers the edges for some cheap
    // anti-aliasing.


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
    z -= length(in.ss_half_width) * scale * w;

    // Apply logarithmic depth buffer 
    z = log(LOG_DEPTH_C * z + 1.0) / log(LOG_DEPTH_C * globals.camera.zfar + 1.0) * w;

    // Feather the edges for anti-aliasing
    let FEATHER_RADIUS: f32 = 1.0;
    let distance = length(in.uv);
    let half_width = in.width / 2.0;
    let full_alpha_radius = 1.0 - (FEATHER_RADIUS / half_width);
    let alpha = 1.0 - (distance - full_alpha_radius) / (1.0 - full_alpha_radius);

    var out: FsPointOut;

    out.depth = z / w;

    // Bitshift the ID left two places, then add the type identifier for surfaces (0b11)
    out.id = (in.point_id << u32(2)) | u32(3);

    return out;
}