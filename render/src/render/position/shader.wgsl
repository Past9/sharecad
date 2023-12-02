struct VertexInput {
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

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) world_position: vec3<f32>,
};

struct Camera {
    view_pos: vec4<f32>,
    view_proj: mat4x4<f32>,
    zfar: f32,
};

@group(0) @binding(0)
var<uniform> camera: Camera;


@vertex
fn vs_main(
    in: VertexInput,
    model_instance: SurfaceModelInstance,
) -> VertexOutput {
    var out: VertexOutput;

    let model_matrix = mat4x4<f32>(
        model_instance.position_matrix_0,
        model_instance.position_matrix_1,
        model_instance.position_matrix_2,
        model_instance.position_matrix_3,
    );

    let world_position = model_matrix * vec4<f32>(in.position, 1.0);

    out.world_position = world_position.xyz;
    out.clip_position = camera.view_proj * world_position;

    // Apply logarithmic depth buffer 
    let c = 1.0;
    out.clip_position.z = log(c * out.clip_position.z + 1.0) / log(c * camera.zfar + 1.0) * out.clip_position.w;

    return out;
}



@fragment
fn fs_main(
    in: VertexOutput
) -> @location(0) vec4<f32> {
    return vec4<f32>(
        in.world_position.x,
        in.world_position.y,
        in.world_position.z,
        1.0
    );
}

