#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct NodeUniform {
    pub offset: [f32; 2],
    pub scale: [f32; 2],
    pub rotation: f32,
    pub opacity: f32,
    pub _padding: [f32; 2],
    pub color: [f32; 4],
}

impl Default for NodeUniform {
    fn default() -> Self {
        Self {
            offset: [0.0, 0.0],
            scale: [1.0, 1.0],
            rotation: 0.0,
            opacity: 1.0,
            _padding: [0.0, 0.0],
            color: [0.2, 0.8, 1.0, 1.0],
        }
    }
}

pub type TransformUniform = NodeUniform;
