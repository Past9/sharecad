//#include "../../shader-includes/globals.wgsl"
//#include "../../shader-includes/vs-curve.wgsl"

@fragment
fn fs_curve(
    in: CurveVertexOut
) -> @location(0) vec4<f32> {
    return vec4<f32>(
        in.world_position.x,
        in.world_position.y,
        in.world_position.z,
        1.0
    );
}