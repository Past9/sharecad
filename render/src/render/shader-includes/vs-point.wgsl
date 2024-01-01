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
    @location(0) world_position: vec3<f32>,
    @location(1) ss_half_width: vec2<f32>,
    @location(2) uv: vec2<f32>,
    @location(3) width: f32,
    @location(4) point_id: u32,
    @location(5) model_id: u32,
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

    out.world_position = world_pos.xyz;
    
    // TODO: This doesn't quite make sense. The half-width in screen space
    // is only necessarily equal to length(x_vec) if the aspect ratio is 1.0.
    // This needs to be adjusted to somehow account for both directions, or
    // allow the fragment shader to account for it.
    out.ss_half_width = ss_half_width;

    out.uv = uv;
    out.width = in.width;

    out.point_id = in.id;
    out.model_id = model_instance.id;

    return out;
}