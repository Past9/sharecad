fn point_adjust_depth(in: PointVertexOut) -> f32 {
    let w = 1.0 / in.clip_position.w;
    var z = in.clip_position.z * w;

    // Get a scaling factor that maps pixels to a depth distance
    var scale = globals.camera.scale.z / sqrt(pow(globals.camera.scale.x, 2.0) + pow(globals.camera.scale.y, 2.0));

    scale *= sqrt(1.0 - pow(length(in.uv), 2.0));

    // Move the Z by `half_width` "pixels" towards the camera
    z -= length(in.ss_half_width) * scale;

    return z / w;
}