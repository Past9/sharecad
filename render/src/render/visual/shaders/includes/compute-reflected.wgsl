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

    let albedo: vec3<f32> = textureSample(t_surface_albedo, s_surface_albedo, in.tex_coords).rgb;
    let surface_normal: vec3<f32> = textureSample(t_surface_normal, s_surface_normal, in.tex_coords).xyz * 2.0 - 1.0;
    let emissive: vec3<f32> = textureSample(t_surface_emissive, s_surface_emissive, in.tex_coords).rgb;
    let roughness: vec3<f32> = textureSample(t_surface_roughness, s_surface_roughness, in.tex_coords).rgb;
    let metallic: vec3<f32> = textureSample(t_surface_metallic, s_surface_metallic, in.tex_coords).rgb;
    let ambient_occlusion: vec3<f32> = textureSample(t_surface_ambient, s_surface_ambient, in.tex_coords).rgb;

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