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
    //@location(2) tangent_light_position: vec3<f32>,
    //@location(3) tangent_view_position: vec3<f32>,
};

struct Camera {
    view_pos: vec4<f32>,
    view_proj: mat4x4<f32>,
    zfar: f32,
};

@group(1) @binding(0)
var<uniform> camera: Camera;

struct Light {
    position: vec3<f32>,
    color: vec3<f32>,
};

@group(2) @binding(0)
var<uniform> light: Light;

const PI = 3.14159265359;

@vertex
fn vs_main(
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

    out.clip_position = camera.view_proj * world_position;
    out.tex_coords = model.tex_coords;
    //out.tangent_position = tangent_matrix * world_position.xyz;
    //out.tangent_view_position = tangent_matrix * camera.view_pos.xyz;
    //out.tangent_light_position = tangent_matrix * light.position;
    out.world_position = model.position.xyz;
    out.world_normal = normalize(normal_matrix * world_normal);
    out.world_tangent = world_tangent;
    out.world_bitangent = world_bitangent;

    // Apply logarithmic depth buffer 
    let c = 1.0;
    out.clip_position.z = log(c * out.clip_position.z + 1.0) / log(c * camera.zfar + 1.0) * out.clip_position.w;

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

@fragment
fn fs_main(
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
    let roughness: f32 = textureSample(t_roughness, s_roughness, in.tex_coords).r;
    let metallic: f32 = textureSample(t_metallic, s_metallic, in.tex_coords).r;
    let ambient_occlusion: vec3<f32> = textureSample(t_ambient, s_ambient, in.tex_coords).rgb;


    let TexCoords = in.tex_coords;
    let WorldPos = in.world_position;
    let Normal = tangent_to_world_matrix * surface_normal;
    let camPos = camera.view_pos.xyz;

    let N = normalize(Normal);
    let V = normalize(camPos - WorldPos);

    var Lo = vec3(0.0);

    // Begin light
    let L = normalize(light.position - WorldPos);
    let H = normalize(V + L);

    let distance = length(light.position - WorldPos);
    let attentuation = 1.0; // 1.0 / (distance * distance) if using falloff lights
    let radiance = light.color * attentuation;

    let f0 = mix(vec3(0.04), albedo, metallic);
    let F = fresnelSchlick(max(dot(H, V), 0.0), f0);

    let NDF = distributionGGX(N, H, roughness);
    let G = geometrySmith(N, V, L, roughness);

    let numerator = NDF * G * F;
    let denominator = 4.0 * max(dot(N, V), 0.0) * max(dot(N, L), 0.0) + 0.0001;
    let specular = numerator / denominator;

    let kS = F;
    var kD = vec3(1.0) - kS;

    kD *= 1.0 - metallic;

    let NdotL = max(dot(N, L), 0.0);
    Lo += (kD * albedo / PI + specular) * radiance * NdotL;

    // End light

    let ambient = vec3(0.4) * albedo * ambient_occlusion;

    let color = ambient + Lo + emissive;


    return vec4<f32>(color, 1.0);
}


fn fresnelSchlick(cosTheta: f32, f0: vec3<f32>) -> vec3<f32> {
    return f0 + (1.0 - f0) * pow(clamp(1.0 - cosTheta, 0.0, 1.0), 5.0);
}

fn distributionGGX(N: vec3<f32>, H: vec3<f32>, roughness: f32) -> f32 {
    let a = roughness * roughness;
    let a2 = a * a;
    let NdotH = max(dot(N, H), 0.0);
    let NdotH2 = NdotH * NdotH;

    let num = a2;
    var denom = (NdotH2 * (a2 - 1.0) + 1.0);
    denom = PI * denom * denom;

    return num / denom;
}

fn geometrySchlickGGX(NdotV: f32, roughness: f32) -> f32 {
    let r = roughness + 1.0;
    let k = (r * r) / 8.0;

    let num = NdotV;
    let denom = NdotV * (1.0 - k) + k;

    return num / denom;
}

fn geometrySmith(N: vec3<f32>, V: vec3<f32>, L: vec3<f32>, roughness: f32) -> f32 {
    let NdotV = max(dot(N, V), 0.0);
    let NdotL = max(dot(N, L), 0.0);
    let ggx2 = geometrySchlickGGX(NdotV, roughness);
    let ggx1 = geometrySchlickGGX(NdotL, roughness);
    return ggx1 * ggx2;
}