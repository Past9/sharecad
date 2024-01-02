//# include ../../shader-includes/globals.wgsl
//# include ../../shader-includes/vs-surface.wgsl

@fragment
fn fs_surface(
    in: SurfaceVertexOut
) -> @location(0) vec4<f32> {
    return vec4<f32>(
        in.world_position.x,
        in.world_position.y,
        in.world_position.z,
        1.0
    );
}