//#include "../../shader-includes/globals.wgsl"
//#include "../../shader-includes/vs-curve.wgsl"
//#include "../../shader-includes/curve-adjust-depth.wgsl"

struct FsCurveOut {
    @location(0) id: u32,
    @builtin(frag_depth) depth: f32,
}

@fragment
fn fs_curve(
    in: CurveVertexOut,
) -> FsCurveOut {
    var out: FsCurveOut;

    // Bitshift the ID left two places, then add the type identifier for surfaces (0b10)
    out.id = (in.curve_id << u32(2)) | u32(2);

    out.depth = curve_adjust_depth(in);

    return out;
}