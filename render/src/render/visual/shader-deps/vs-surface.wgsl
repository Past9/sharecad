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
    @location(0) tex_coords: vec2<f32>,
    @location(1) world_position: vec3<f32>,
    @location(2) world_normal: vec3<f32>,
    @location(3) world_tangent: vec3<f32>,
    @location(4) world_bitangent: vec3<f32>,
};

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

    let normal_matrix = mat3x3<f32>(
        model_instance.direction_matrix_0,
        model_instance.direction_matrix_1,
        model_instance.direction_matrix_2,
    );

    // Construct the tangent matrix
    let world_normal = normalize(normal_matrix * in.normal);
    let world_tangent = normalize(normal_matrix * in.tangent);
    let world_bitangent = normalize(normal_matrix * in.bitangent);
    let tangent_matrix = transpose(mat3x3<f32>(
        world_tangent,
        world_bitangent,
        world_normal,
    ));

    let world_position = model_matrix * vec4<f32>(in.position, 1.0);

    out.clip_position = globals.camera.view_proj * world_position;
    out.tex_coords = in.tex_coords;
    out.world_position = in.position.xyz;
    out.world_normal = normalize(normal_matrix * world_normal);
    out.world_tangent = world_tangent;
    out.world_bitangent = world_bitangent;


    return out;
}