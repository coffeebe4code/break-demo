struct VertexInput {
    @location(0) position: vec2<f32>,
    @location(1) tex_coords: vec2<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
}

@group(0) @binding(0)
var<uniform> time: f32;

@vertex
fn vs_main(
    model: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;
    out.tex_coords = model.tex_coords;

    let cos_theta = cos(time);
    let sin_theta = sin(time);

    // Rotation matrix applied to texture coordinates
    let rotated_tex_coords = vec2<f32>(
        cos_theta * model.tex_coords.x - sin_theta * model.tex_coords.y,
        sin_theta * model.tex_coords.x + cos_theta * model.tex_coords.y
    );

    out.tex_coords = rotated_tex_coords;
    out.clip_position = vec4<f32>(model.position,0.0, 1.0);
    return out;
}

@group(1) @binding(0)
var t_diffuse: texture_2d<f32>;
@group(1) @binding(1)
var s_diffuse: sampler;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return textureSample(t_diffuse, s_diffuse, in.tex_coords);
}
