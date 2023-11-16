struct VertexIn {
    @location(0) pos: vec2<f32>,
    @location(1) dir: vec2<f32>,
};

struct VertexOut {
    @builtin(position) position: vec4<f32>,
    @location(0) color: vec4<f32>,
    @location(1) uv: vec2<f32>,
    @location(2) length: f32,
};

struct Uniforms {
    @size(16) angle: f32 //pad to 16 bytes
};

@group(0) @binding(0) 
var<uniform> uniforms: Uniforms;

// Half the width of the line
const half_width: f32 = 0.0;

@vertex
fn vs_main(@builtin(vertex_index) v_idx: u32, v: VertexIn) -> VertexOut {
    var out: VertexOut;

    // Determine whether we need to push the vertex left or right.
    // They'll be pushed left (relative to the line segment's
    // direction of travel) by default. Every second vertex gets 
    // pushed to the right. We create the `flip` variable to reverse 
    // the `orth` vector for this purpose. 
    let v_idx_i = i32(v_idx);
    var flip_orth: f32 = 1.0;
    if v_idx_i % 2 == 1 {
        flip_orth = -1.0;
    }

    // Get a vector perpendicular to the line's direction
    // of travel, and flip it if needed. Make its magnitude 
    // the half width of the line.
    let orth = normalize(vec2<f32>(-v.dir.y, v.dir.x)) * flip_orth * half_width;

    // Determine whether we need to expand in the direction of the line's 
    // travel or the opposite of it. The first two of every four vertices
    // go opposite, and the second pair goes forward.
    var flip_travel = 1.0;
    if v_idx_i % 4 < 2 {
        flip_travel = -1.0;
    }

    // Get a vector along the direction of travel but scaled to 
    // the half width
    let travel = normalize(v.dir) * flip_travel * half_width;

    // Move the vertex along that vector 
    var position = v.pos + orth + travel;
    position.x = position.x * cos(uniforms.angle);

    out.position = vec4<f32>(position, 0.0, 1.0);
    out.color = vec4<f32>(0.0, 0.0, 0.0, 1.0);

    // Use the flip flags to assign (u, v) coordinates to the vertices. We'll 
    // use these in the fragment shader to make transparent rounded corners 
    // so line segments blend together nicely. The u coordinate goes along 
    // the (fully expanded) line's direction of travel and starts at -1.0 and 
    // goes to 1.0. The v coordinate starts at -1.0 from the left of the 
    // expanded line and goes to 1.0 on the right. 
    out.uv = vec2<f32>(flip_travel, flip_orth);

    // Output the length of the original line. We'll need this to scale the
    // UV coordinates in the fragment shader.
    out.length = length(v.dir);

    return out;
}

@fragment
fn fs_main(in: VertexOut) -> @location(0) vec4<f32> {
    var color = in.color;

    // Half the length of the original non-expanded line
    let half_length = in.length / 2.0;

    // Get the U passed in from the vertex shader, but we're going to change 
    // it so it represents distance in the U-direction from the endpoints
    // of the unexpanded line, not from the center of the line. 
    var u = abs(in.uv.x);

    // Get the U-coordinate where the line ends (before the forward/backward extension)
    // (positive only, we'll be owrking with distance and absolute values so it's fine)
    let line_end_u = half_length / (half_length + half_width);

    // Get how much of U is past that point
    u = u - line_end_u;

    // Now scale that so it's between 0.0 and 1.0 again.
    u = u / (1.0 - line_end_u); 

    // Get V
    let v = abs(in.uv.y);

    color = vec4<f32>(1.0, 1.0, 1.0, 1.0);
    if u > 0.0 && sqrt(pow(u, 2.0) + pow(v, 2.0)) > 1.0 {
        color.a = 0.0;
    }

    return color;
}
