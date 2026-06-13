#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub position: [f32; 2],
}

impl Vertex {
    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        const ATTRIBUTES: [wgpu::VertexAttribute; 1] =
            wgpu::vertex_attr_array![0 => Float32x2];

        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &ATTRIBUTES,
        }
    }
}

pub fn quad_vertices() -> Vec<Vertex> {
    vec![
        Vertex { position: [-0.5, -0.5] },
        Vertex { position: [ 0.5, -0.5] },
        Vertex { position: [ 0.5,  0.5] },
        Vertex { position: [-0.5, -0.5] },
        Vertex { position: [ 0.5,  0.5] },
        Vertex { position: [-0.5,  0.5] },
    ]
}
