use std::fs;
use std::path::{Path, PathBuf};

use crate::offscreen::{FrameRequest, OffscreenRenderer, RenderSize};

#[derive(Clone, Debug)]
pub struct ExportSettings {
    pub fps: u32,
    pub width: u32,
    pub height: u32,
    pub duration_seconds: f64,
}

impl ExportSettings {
    pub fn frame_count(&self) -> u64 {
        (self.duration_seconds * self.fps as f64).ceil() as u64
    }
}

pub fn export_png_sequence_stub(output_dir: impl AsRef<Path>, settings: ExportSettings) -> Result<Vec<PathBuf>, String> {
    let output_dir = output_dir.as_ref();
    fs::create_dir_all(output_dir).map_err(|error| format!("failed to create output dir: {error}"))?;

    let renderer = OffscreenRenderer::new();
    let mut frames = Vec::new();

    for frame_index in 0..settings.frame_count() {
        let request = FrameRequest {
            frame_index,
            fps: settings.fps,
            size: RenderSize { width: settings.width, height: settings.height },
        };
        let rgba = renderer.render_frame_rgba(request);
        let frame_path = output_dir.join(format!("frame_{frame_index:06}.rgba"));
        fs::write(&frame_path, rgba).map_err(|error| format!("failed to write frame: {error}"))?;
        frames.push(frame_path);
    }

    Ok(frames)
}
