//# include ../../shader-includes/globals.wgsl
//# include ../../shader-includes/vs-surface.wgsl
//# include includes/pi.wgsl
//# include includes/surface-material.wgsl
//# include includes/lights.wgsl
//# include includes/compute-reflected.wgsl

@fragment
fn fs_opaque_surface(
    @builtin(front_facing) front_facing: bool,
    in: SurfaceVertexOut
) -> @location(0) vec4<f32> {
    // Calculate lighting
    var color = compute_reflected(front_facing, in, vec3(0.0));

    // Apply tint (TODO: use this for selection)
    let tint = vec4(0.0, 0.0, 0.0, 0.0);
    color = (1.0 - tint.a) * color + (tint.rgb * tint.a);

    return vec4<f32>(color, 1.0);
}