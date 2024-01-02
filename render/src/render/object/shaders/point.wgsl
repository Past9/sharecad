//#include ../../shader-includes/globals.wgsl
//#include ../../shader-includes/vs-point.wgsl
//#include ../../shader-includes/point-adjust-depth.wgsl

struct FsPointOut {
    @location(0) id: u32,
    @builtin(frag_depth) depth: f32,
}

@fragment
fn fs_point(
    in: PointVertexOut,
) -> FsPointOut {
    if length(in.uv) > 1.0 {
        discard;
    }

    var out: FsPointOut;

    out.depth = point_adjust_depth(in);

    // Bitshift the ID left two places, then add the type identifier for surfaces (0b11)
    out.id = (in.point_id << u32(2)) | u32(3);

    return out;
}