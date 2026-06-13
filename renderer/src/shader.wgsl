struct NodeUniform {
    offset: vec2<f32>,
    scale: vec2<f32>,
    rotation: f32,
    opacity: f32,
    padding: vec2<f32>,
    color: vec4<f32>,
};

@group(0) @binding(0)
var<uniform> u_node: NodeUniform;

struct VertexInput {
    @location(0) position: vec2<f32>,
};

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
};

@vertex
fn vs_main(input: VertexInput) -> VertexOutput {
    var out: VertexOutput;

    let s = sin(u_node.rotation);
    let c = cos(u_node.rotation);
    let scaled = input.position * u_node.scale;
    let rotated = vec2<f32>(
        scaled.x * c - scaled.y * s,
        scaled.x * s + scaled.y * c
    );
    let pos = rotated + u_node.offset;

    out.position = vec4<f32>(pos, 0.0, 1.0);
    return out;
}

@fragment
fn fs_main() -> @location(0) vec4<f32> {
    return vec4<f32>(u_node.color.rgb, u_node.color.a * u_node.opacity);
}
