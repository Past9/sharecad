struct Globals {
    @align(16) num_directional_lights: u32,
    @align(16) num_ambient_lights: u32,
    @align(16) camera: Camera
}

struct Camera {
    view_pos: vec4<f32>,
    view_proj: mat4x4<f32>,
    zfar: f32,
};

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) tex_coords: vec2<f32>,
    @location(2) normal: vec3<f32>,
    @location(3) tangent: vec3<f32>,
    @location(4) bitangent: vec3<f32>,
    @location(5) param_coords: vec2<f32>,
};

struct InstanceInput {
    @location(6) model_matrix_0: vec4<f32>,
    @location(7) model_matrix_1: vec4<f32>,
    @location(8) model_matrix_2: vec4<f32>,
    @location(9) model_matrix_3: vec4<f32>,

    @location(10) normal_matrix_0: vec3<f32>,
    @location(11) normal_matrix_1: vec3<f32>,
    @location(12) normal_matrix_2: vec3<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
    @location(1) world_position: vec3<f32>,
    @location(2) world_normal: vec3<f32>,
    @location(3) world_tangent: vec3<f32>,
    @location(4) world_bitangent: vec3<f32>,
};

@group(1) @binding(0)
var<uniform> globals: Globals;

const PI = 3.14159265359;

@vertex
fn vs_surface(
    model: VertexInput,
    instance: InstanceInput,
) -> VertexOutput {
    var out: VertexOutput;

    let model_matrix = mat4x4<f32>(
        instance.model_matrix_0,
        instance.model_matrix_1,
        instance.model_matrix_2,
        instance.model_matrix_3,
    );

    let normal_matrix = mat3x3<f32>(
        instance.normal_matrix_0,
        instance.normal_matrix_1,
        instance.normal_matrix_2,
    );

    // Construct the tangent matrix
    let world_normal = normalize(normal_matrix * model.normal);
    let world_tangent = normalize(normal_matrix * model.tangent);
    let world_bitangent = normalize(normal_matrix * model.bitangent);
    let tangent_matrix = transpose(mat3x3<f32>(
        world_tangent,
        world_bitangent,
        world_normal,
    ));

    let world_position = model_matrix * vec4<f32>(model.position, 1.0);

    out.clip_position = globals.camera.view_proj * world_position;
    out.tex_coords = model.tex_coords;
    out.world_position = model.position.xyz;
    out.world_normal = normalize(normal_matrix * world_normal);
    out.world_tangent = world_tangent;
    out.world_bitangent = world_bitangent;

    // Apply logarithmic depth buffer 
    let c = 1.0;
    out.clip_position.z = log(c * out.clip_position.z + 1.0) / log(c * globals.camera.zfar + 1.0) * out.clip_position.w;

    return out;
}

@group(0) @binding(0)
var t_albedo: texture_2d<f32>;
@group(0) @binding(1)
var s_albedo: sampler;
@group(0) @binding(2)
var t_normal: texture_2d<f32>;
@group(0) @binding(3)
var s_normal: sampler;
@group(0) @binding(4)
var t_emissive: texture_2d<f32>;
@group(0) @binding(5)
var s_emissive: sampler;
@group(0) @binding(6)
var t_roughness: texture_2d<f32>;
@group(0) @binding(7)
var s_roughness: sampler;
@group(0) @binding(8)
var t_metallic: texture_2d<f32>;
@group(0) @binding(9)
var s_metallic: sampler;
@group(0) @binding(10)
var t_ambient: texture_2d<f32>;
@group(0) @binding(11)
var s_ambient: sampler;
@group(0) @binding(12)
var t_transmit: texture_2d<f32>;
@group(0) @binding(13)
var s_transmit: sampler;

@group(2) @binding(0) 
var<storage, read> directional_lights: array<DirectionalLight>;

@group(2) @binding(1) 
var<storage, read> ambient_lights: array<AmbientLight>;

struct DirectionalLight {
    @location(0) direction: vec3<f32>,
    @location(1) color: vec3<f32>,
};

struct AmbientLight {
    @location(0) color: vec3<f32>,
};

@fragment
fn fs_opaque_surface(
    in: VertexOutput
) -> @location(0) vec4<f32> {
    // Converts from tangent space to world space
    let tangent_to_world_matrix = mat3x3<f32>(
        in.world_tangent,
        in.world_bitangent,
        in.world_normal,
    );

    let albedo: vec3<f32> = textureSample(t_albedo, s_albedo, in.tex_coords).rgb;
    let surface_normal: vec3<f32> = textureSample(t_normal, s_normal, in.tex_coords).xyz * 2.0 - 1.0;
    let emissive: vec3<f32> = textureSample(t_emissive, s_emissive, in.tex_coords).rgb;
    let roughness: vec3<f32> = textureSample(t_roughness, s_roughness, in.tex_coords).rgb;
    let metallic: vec3<f32> = textureSample(t_metallic, s_metallic, in.tex_coords).rgb;
    let ambient_occlusion: vec3<f32> = textureSample(t_ambient, s_ambient, in.tex_coords).rgb;
    let transmit: vec3<f32> = textureSample(t_transmit, s_transmit, in.tex_coords).rgb;



    var reflected = vec3(0.0);

    // Directional lights
    for (var i: u32 = u32(0); i < globals.num_directional_lights; i++) {
        reflected += pbr(
            albedo,
            tangent_to_world_matrix * surface_normal,
            emissive,
            roughness,
            metallic,
            -directional_lights[i].direction,
            globals.camera.view_pos.xyz - in.world_position,
            1.0,
            directional_lights[i].color
        );
    }

    // Ambient lights
    for (var i: u32 = u32(0); i < globals.num_ambient_lights; i++) {
        reflected += ambient_lights[i].color * albedo * ambient_occlusion;
    }

    // Texture emission
    var color = reflected + emissive;

    color = color / (color + vec3(1.0));
    color = pow(color, vec3(1.0 / 2.2));


    return vec4<f32>(color, transmit.r);
}

@fragment
fn fs_translucent_surface(
    in: VertexOutput
) -> @location(0) vec4<f32> {
    return vec4<f32>(1.0, 0.0, 0.0, 1.0);
}

fn pbr(
    albedo: vec3<f32>,
    normal: vec3<f32>,
    emissive: vec3<f32>,
    roughness: vec3<f32>,
    metallic: vec3<f32>,
    surf_to_light: vec3<f32>,
    surf_to_camera: vec3<f32>,
    attentuation: f32,
    light_color: vec3<f32>
) -> vec3<f32> {
    let n = normalize(normal);
    let v = normalize(surf_to_camera);
    let l = normalize(surf_to_light);
    let h = normalize(v + l);

    let radiance = light_color * attentuation;

    let f0 = mix(vec3(0.04), albedo, metallic);
    let f = fresnel_schlick(max(dot(h, v), 0.0), f0);

    let ndf = distribution_ggx(n, h, roughness);
    let g = geometry_smith(n, v, l, roughness);

    let numerator = ndf * g * f;
    let denominator = 4.0 * max(dot(n, v), 0.0) * max(dot(n, l), 0.0) + 0.0001;
    let specular = numerator / denominator;

    let ks = f;
    var kd = vec3(1.0) - ks;

    kd *= 1.0 - metallic;

    let n_dot_l = max(dot(n, l), 0.0);
    return (kd * albedo / PI + specular) * radiance * n_dot_l;
}

fn fresnel_schlick(cos_theta: f32, f0: vec3<f32>) -> vec3<f32> {
    return f0 + (1.0 - f0) * pow(clamp(1.0 - cos_theta, 0.0, 1.0), 5.0);
}

fn distribution_ggx(n: vec3<f32>, h: vec3<f32>, roughness: vec3<f32>) -> vec3<f32> {
    let a = roughness * roughness;
    let a2 = a * a;
    let n_dot_h = max(dot(n, h), 0.0);
    let n_dot_h_2 = n_dot_h * n_dot_h;

    let num = a2;
    var denom = (n_dot_h_2 * (a2 - 1.0) + 1.0);
    denom = PI * denom * denom;

    return num / denom;
}

fn geometry_schlick_ggx(n_dot_v: f32, roughness: vec3<f32>) -> vec3<f32> {
    let r = roughness + 1.0;
    let k = (r * r) / 8.0;

    let num = n_dot_v;
    let denom = n_dot_v * (1.0 - k) + k;

    return num / denom;
}

fn geometry_smith(n: vec3<f32>, v: vec3<f32>, l: vec3<f32>, roughness: vec3<f32>) -> vec3<f32> {
    let n_dot_v = max(dot(n, v), 0.0);
    let n_dot_l = max(dot(n, l), 0.0);
    let ggx2 = geometry_schlick_ggx(n_dot_v, roughness);
    let ggx1 = geometry_schlick_ggx(n_dot_l, roughness);
    return ggx1 * ggx2;
}