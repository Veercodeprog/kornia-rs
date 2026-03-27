//! **Live panorama** pipeline (stub).
//!
//! Full version: feature detection + matching + warp + blend across frames or cameras;
//! GPU acceleration for warp/resample in **kornia-rs**.

use crate::{ComputeBackend, EdgeVisionError, FrameView, ProcessedFrame, VisionPipeline};

#[derive(Debug)]
pub struct PanoramaPipeline {
    pub backend: ComputeBackend,
    /// How many past keyframes to keep (proposal parameter).
    pub ring_buffer_len: usize,
}

impl PanoramaPipeline {
    pub fn new(backend: ComputeBackend) -> Self {
        Self {
            backend,
            ring_buffer_len: 8,
        }
    }
}

impl VisionPipeline for PanoramaPipeline {
    fn id(&self) -> &'static str {
        "live_panorama"
    }

    fn process(&mut self, input: FrameView<'_>) -> Result<ProcessedFrame, EdgeVisionError> {
        let _ = (self.backend, self.ring_buffer_len);
        if input.data.is_empty() {
            return Err(EdgeVisionError::Config(
                "empty frame buffer (stub requires non-empty slice)".into(),
            ));
        }
        Err(EdgeVisionError::NotImplemented(
            "feature match + cylindrical/spherical warp + multi-band blend (GPU resample in kornia-rs)",
        ))
    }
}
