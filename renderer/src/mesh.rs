// Step E1: Basic mesh (quad) for scene graph rendering

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Vertex {
    pub position: [f32; 2],
}

impl Vertex {
    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        use std::mem;
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x2,
                }
            ],
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