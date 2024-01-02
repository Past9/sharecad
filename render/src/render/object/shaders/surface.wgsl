//# include ../../shader-includes/globals.wgsl
//# include ../../shader-includes/vs-surface.wgsl

@fragment
fn fs_surface(
    in: SurfaceVertexOut
) -> @location(0) u32 {
    // Bitshift the ID left two places, then add the type identifier for surfaces (0b01)
    return (in.surface_id << u32(2)) | u32(1);
}