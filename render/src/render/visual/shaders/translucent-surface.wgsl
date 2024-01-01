//#include "includes/globals.wgsl"
//#include "includes/vs-surface.wgsl"
//#include "includes/pi.wgsl"
//#include "includes/surface-material.wgsl"
//#include "includes/lights.wgsl"
//#include "includes/compute-reflected.wgsl"

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

    let transmit: vec3<f32> = textureSample(t_surface_transmit, s_surface_transmit, in.tex_coords).rgb;
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