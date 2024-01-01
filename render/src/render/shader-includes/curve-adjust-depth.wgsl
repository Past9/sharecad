

fn curve_adjust_depth(in: CurveVertexOut) -> f32 {
    // Adjust the z-depth of the fragment so that it's closer to the camera by 
    // the half_width below:

    // Get the actual W, not the weird reciprocal that OpenGL provides for some reason.
    let w = 1.0 / in.clip_position.w;

    // Reverse-transform Z
    var z = in.clip_position.z * w;

    // Get a scaling factor that maps pixels to a depth distance
    var scale = 2.0 * globals.camera.scale.z / sqrt(pow(globals.camera.scale.x, 2.0) + pow(globals.camera.scale.y, 2.0));

    // Move the Z by `half_width` "pixels" towards the camera
    z -= in.ss_half_width * scale;

    return z / w;
}