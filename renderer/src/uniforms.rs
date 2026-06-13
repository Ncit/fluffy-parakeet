// Step E1: Transform uniforms for scene graph animation

use wgpu::util::DeviceExt;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct TransformUniform {
    pub offset: [f32; 2],
    pub scale: [f32; 2],
}

unsafe impl bytemuck::Pod for TransformUniform {}
unsafe impl bytemuck::Zeroable for TransformUniform {}

pub struct UniformBuffer {
    pub buffer: wgpu::Buffer,
}

impl UniformBuffer {
    pub fn new(device: &wgpu::Device, transform: TransformUniform) -> Self {
        let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Transform Uniform Buffer"),
            contents: bytemuck::bytes_of(&transform),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        Self { buffer }
    }
}