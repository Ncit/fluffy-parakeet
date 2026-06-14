#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RenderSize {
    pub width: u32,
    pub height: u32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FrameRequest {
    pub frame_index: u64,
    pub fps: u32,
    pub size: RenderSize,
}

impl FrameRequest {
    pub fn timestamp_seconds(&self) -> f64 {
        self.frame_index as f64 / self.fps as f64
    }
}

pub struct OffscreenRenderer;

impl OffscreenRenderer {
    pub fn new() -> Self {
        Self
    }

    pub fn render_frame_rgba(&self, request: FrameRequest) -> Vec<u8> {
        let pixel_count = request.size.width as usize * request.size.height as usize;
        vec![0; pixel_count * 4]
    }
}
