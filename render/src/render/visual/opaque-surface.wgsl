//#include "shader-deps/globals.wgsl"
//#include "shader-deps/camera.wgsl"
//#include "shader-deps/vs-surface.wgsl"
//#include "shader-deps/pi.wgsl"
//#include "shader-deps/surface-material.wgsl"
//#include "shader-deps/lights.wgsl"
//#include "shader-deps/compute-reflected.wgsl"

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