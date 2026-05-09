struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
}

@group(0) @binding(0) var t_diffuse: texture_2d<f32>;
@group(0) @binding(1) var s_diffuse: sampler;

@vertex
fn vs_main(@location(0) position: vec2<f32>,
           @location(1) tex_coords: vec2<f32>) -> VertexOutput {
    var out: VertexOutput;
    out.position = vec4<f32>(position, 0.0, 1.0);
    out.tex_coords = tex_coords;
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let sample = textureSample(t_diffuse, s_diffuse, in.tex_coords).r;
    let color = select(vec4<f32>(0.0, 0.0, 0.0, 1.0),
                       vec4<f32>(1.0, 1.0, 1.0, 1.0),
                       sample > 0.5);
    return color;
}
