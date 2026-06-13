struct Uniforms {
    offset: vec2<f32>,
    scale: vec2<f32>,
};

@group(0) @binding(0)
var<uniform> u_transform: Uniforms;

struct VertexInput {
    @location(0) position: vec2<f32>,
};

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
};

@vertex
fn vs_main(input: VertexInput) -> VertexOutput {
    var out: VertexOutput;

    let pos = input.position * u_transform.scale + u_transform.offset;
    out.position = vec4<f32>(pos, 0.0, 1.0);
    return out;
}

@fragment
fn fs_main() -> @location(0) vec4<f32> {
    return vec4<f32>(0.2, 0.8, 1.0, 1.0);
}