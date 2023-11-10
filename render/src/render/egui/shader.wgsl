struct VertexInput {
    @location(0) position: vec2<f32>,
    @location(1) tex_coord: vec2<f32>,
};

struct FragInput {
    @builtin(position) position: vec4<f32>,
    @location(0) tex_coord: vec2<f32>,
};


@vertex
fn vs_main(
    in: VertexInput,
) -> FragInput {
    var out: FragInput;
    out.position = vec4<f32>(in.position, 0.0, 1.0);
    out.tex_coord = in.tex_coord;
    return out;
}

@group(0) @binding(0)
var t_diffuse: texture_2d<f32>;
@group(0) @binding(1)
var s_diffuse: sampler;

@fragment
fn fs_main(
    in: FragInput
) -> @location(0) vec4<f32> {
    return textureSample(t_diffuse, s_diffuse, in.tex_coord);
    //return vec4<f32>(1.0, 0.0, 0.0, 1.0);
}