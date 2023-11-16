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

struct SurfaceVertexIn {
    @location(0) position: vec3<f32>,
    @location(1) tex_coords: vec2<f32>,
    @location(2) normal: vec3<f32>,
    @location(3) tangent: vec3<f32>,
    @location(4) bitangent: vec3<f32>,
    @location(5) param_coords: vec2<f32>,
};

struct SurfaceInstanceIn {
    @location(6) model_matrix_0: vec4<f32>,
    @location(7) model_matrix_1: vec4<f32>,
    @location(8) model_matrix_2: vec4<f32>,
    @location(9) model_matrix_3: vec4<f32>,

    @location(10) normal_matrix_0: vec3<f32>,
    @location(11) normal_matrix_1: vec3<f32>,
    @location(12) normal_matrix_2: vec3<f32>,
}

struct SurfaceVertexOut {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
    @location(1) world_position: vec3<f32>,
    @location(2) world_normal: vec3<f32>,
    @location(3) world_tangent: vec3<f32>,
    @location(4) world_bitangent: vec3<f32>,
};

struct CurveVertexIn {
    @location(0) position: vec3<f32>,
    @location(1) direction: vec3<f32>,
    @location(2) width: f32,
};

struct CurveInstanceIn {
    @location(3) position_matrix_0: vec4<f32>,
    @location(4) position_matrix_1: vec4<f32>,
    @location(5) position_matrix_2: vec4<f32>,
    @location(6) position_matrix_3: vec4<f32>,

    @location(7) direction_matrix_0: vec3<f32>,
    @location(8) direction_matrix_1: vec3<f32>,
    @location(9) direction_matrix_2: vec3<f32>,
};

struct CurveVertexOut {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec4<f32>,
    @location(1) uv: vec2<f32>,
    @location(2) length: f32,
    @location(3) width: f32,
    @location(4) direction: vec3<f32>
}

@group(1) @binding(0)
var<uniform> globals: Globals;

const PI = 3.14159265359;

@vertex
fn vs_surface(
    model: SurfaceVertexIn,
    instance: SurfaceInstanceIn,
) -> SurfaceVertexOut {
    var out: SurfaceVertexOut;

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

@vertex
fn vs_curve(
    @builtin(vertex_index) v_idx: u32,
    model: CurveVertexIn,
    instance: CurveInstanceIn,
) -> CurveVertexOut {
    let v_idx_i = i32(v_idx);
    let index = v_idx_i % 4;
    let is_start = index <= 1;
    let half_width = model.width / 2.0;

    var out: CurveVertexOut;

    let position_matrix = mat4x4<f32>(
        instance.position_matrix_0,
        instance.position_matrix_1,
        instance.position_matrix_2,
        instance.position_matrix_3,
    );

    let direction_matrix = mat3x3<f32>(
        instance.direction_matrix_0,
        instance.direction_matrix_1,
        instance.direction_matrix_2,
    );

    // Works up to here
    //out.clip_position = globals.camera.view_proj * (position_matrix * vec4(model.position, 1.0));
    //return out;

    // Depending on the index, the current vertex is either at the start 
    // or end of the line segment. Using this information and the line's 
    // direction vector in world space, we find the world space coordinates
    // of both the start and end points of the segment.
    var start = vec3(0.0);
    var end = vec3(0.0);
    if is_start {
        start = model.position;
        end = model.position + model.direction;
    } else {
        start = model.position - model.direction;
        end = model.position;
    }


    // Transform the start and end points into clip space
    let start_clip_pos = globals.camera.view_proj * (position_matrix * vec4(start, 1.0));
    let end_clip_pos = globals.camera.view_proj * (position_matrix * vec4(end, 1.0));

    // Transform the start and end points into screen space
    let start_screen_pos = (start_clip_pos / start_clip_pos.w).xyz;
    let end_screen_pos = (end_clip_pos / end_clip_pos.w).xyz;

    // Get the direction of the line in screen space
    let direction = end_screen_pos - start_screen_pos;

    // Now that we have a screen space direction, we can expand the vertices
    // to form a camera-aligned quad of the desired width.

    // Get a vector perpendicular to the line direction and flip it if needed. 
    // Make its magnitude half the desired line width.
    var flip_orth: f32 = 1.0;
    if v_idx_i % 2 == 1 {
        flip_orth = -1.0;
    }
    let orth = normalize(vec2(-direction.y, direction.x)) * flip_orth * half_width; 

    // Get a vector along the line's direction and flip it if needed. Make its magnitude 
    // half the desired line width.
    var flip_travel: f32 = 1.0;
    if is_start {
        flip_travel = -1.0;
    }
    let travel = normalize(direction.xy) * flip_travel * half_width;

    // Move the vertex along those vectors
    var final_pos = vec3(0.0);
    if is_start {
        final_pos = start_screen_pos;
    } else {
        final_pos = end_screen_pos;
    }
    final_pos += vec3(orth, 0.0) + vec3(travel, 0.0);
    
    // Set the output clip position for the current point
    out.clip_position = vec4(final_pos, 1.0);

    // Set the screen space direction
    out.direction = direction;

    out.uv = vec2(flip_travel, flip_orth);
    out.length = length(end_screen_pos.xy - start_screen_pos.xy);

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
    @builtin(front_facing) front_facing: bool,
    in: SurfaceVertexOut
) -> @location(0) vec4<f32> {
    var color = compute_reflected(front_facing, in, vec3(0.0));

    //color = color / (color + vec3(1.0));
    //color = pow(color, vec3(1.0 / 2.2));

    return vec4<f32>(color, 1.0);
}

@fragment
fn fs_opaque_curve(
    in: CurveVertexOut
) -> @location(0) vec4<f32> {
    var color = in.color;

    // Half the length of the original non-expanded line
    let half_length = in.length / 2.0;

    // Get the U passed in from the vertex shader, but we're going to change 
    // it so it represents distance in the U-direction from the endpoints
    // of the unexpanded line, not from the center of the line. 
    var u = abs(in.uv.x);

    // Get the U-coordinate where the line ends (before the forward/backward extension)
    // (positive only, we'll be owrking with distance and absolute values so it's fine)
    let line_end_u = half_length / (half_length + in.width);

    // Get how much of U is past that point
    u = u - line_end_u;

    // Now scale that so it's between 0.0 and 1.0 again.
    u = u / (1.0 - line_end_u); 

    // Get V
    let v = abs(in.uv.y);

    color = vec4<f32>(1.0, 1.0, 1.0, 1.0);
    if u > 0.0 && sqrt(pow(u, 2.0) + pow(v, 2.0)) > 1.0 {
        color.a = 0.0;
    }

    //return color;
    return vec4<f32>(vec3(0.0), 1.0);
}

struct TranslucentOutput {
    @location(0) accum: vec4<f32>,
    @location(1) transmit: vec3<f32>,
    @location(2) background: vec4<f32>,
}

@fragment
fn fs_translucent_surface(
    @builtin(front_facing) front_facing: bool,
    in: SurfaceVertexOut,
) -> TranslucentOutput {

    let transmit: vec3<f32> = textureSample(t_transmit, s_transmit, in.tex_coords).rgb;
    var reflected = compute_reflected(front_facing, in, transmit);

    var surface_color = vec4(reflected, 1.0);

    var out: TranslucentOutput;
    out.background = vec4(surface_color.a * (vec3(1.0) - transmit), 1.0);

    // Calculate transparency
    surface_color.a *= 1.0 - clamp((transmit.r + transmit.g + transmit.b) / 3.0, 0.0, 1.0);
    let a = min(1.0, surface_color.a) * 8.0 + 0.01;
    let b = in.clip_position.z * 2.0;

    let w = clamp(a * a * a * 1e8 * b * b * b, 1e-2, 3e2);

    out.accum = surface_color * w;
    out.transmit = vec3(surface_color.a);

    return out;
}

fn compute_reflected(
    front_facing: bool,
    in: SurfaceVertexOut,
    transmit: vec3<f32>
) -> vec3<f32> {
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

    var reflected = vec3(0.0);

    var normal = tangent_to_world_matrix * surface_normal;
    if !front_facing {
        normal = -normal;
    }

    // Directional lights
    for (var i: u32 = u32(0); i < globals.num_directional_lights; i++) {
        let color = directional_lights[i].color * (1.0 - transmit);
        reflected += pbr(
            albedo,
            normal,
            emissive,
            roughness,
            metallic,
            -directional_lights[i].direction,
            globals.camera.view_pos.xyz - in.world_position,
            1.0,
            color
        );
    }

    // Ambient lights
    for (var i: u32 = u32(0); i < globals.num_ambient_lights; i++) {
        reflected += ambient_lights[i].color * albedo * ambient_occlusion;
    }

    return reflected + emissive;
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


struct ScreenVertexIn {
    @location(0) position: vec2<f32>,
    @location(1) tex_coord: vec2<f32>,
};

struct ScreenVertexOut {
    @builtin(position) position: vec4<f32>,
    @location(0) tex_coord: vec2<f32>,
};

@vertex
fn vs_composite(
    in: ScreenVertexIn,
) -> ScreenVertexOut {
    var out: ScreenVertexOut;
    out.position = vec4<f32>(in.position, 0.0, 1.0);
    out.tex_coord = in.tex_coord;
    return out;
}

@group(0) @binding(0)
var t_opaque_target: texture_2d<f32>;
@group(0) @binding(1)
var s_opaque_target: sampler;
@group(0) @binding(2)
var t_accum_target: texture_2d<f32>;
@group(0) @binding(3)
var s_accum_target: sampler;
@group(0) @binding(4)
var t_transmit_target: texture_2d<f32>;
@group(0) @binding(5)
var s_transmit_target: sampler;

fn max_component3(v: vec3<f32>) -> f32 {
    return max(max(v.x, v.y), v.z);
}

fn max_component4(v: vec4<f32>) -> f32 {
    return max(max(max(v.x, v.y), v.z), v.w);
}

@fragment
fn fs_composite(
    in: ScreenVertexOut
) -> @location(0) vec4<f32> {
    var color_background = textureSample(t_opaque_target, s_opaque_target, in.tex_coord).rgb;
    var color_transmit = textureSample(t_transmit_target, s_transmit_target, in.tex_coord).r;
    var color_accum = textureSample(t_accum_target, s_accum_target, in.tex_coord).rgba;

    let avg_color = color_accum.rgb / max(color_accum.a, 0.00001);

    var accum_part = vec3(0.0);
    if color_accum.a != 0.0 {
        accum_part = color_accum.rgb / color_accum.a;
    }

    var color = accum_part * (1.0 - color_transmit) + color_background;

    return vec4(color, 1.0);
}

