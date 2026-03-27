//! **Bird's-eye / IPM** pipeline (stub).
//!
//! Full version: ground homography or full intrinsics + extrinsics → remap via **kornia-rs**
//! (CPU baseline + GPU path in `kornia-imgproc` / tensors).

use crate::{ComputeBackend, EdgeVisionError, FrameView, ProcessedFrame, VisionPipeline};

/// Carries calibration placeholders (paths or inline params) for a proposal story.
#[derive(Clone, Debug, Default)]
pub struct BirdsEyeCalibration {
    /// e.g. path to 3x3 homography JSON or NPZ — not loaded in this stub.
    pub homography_path: Option<String>,
}

#[derive(Debug)]
pub struct BirdsEyePipeline {
    pub backend: ComputeBackend,
    pub calibration: BirdsEyeCalibration,
}

impl BirdsEyePipeline {
    pub fn new(backend: ComputeBackend) -> Self {
        Self {
            backend,
            calibration: BirdsEyeCalibration::default(),
        }
    }
}

impl VisionPipeline for BirdsEyePipeline {
    fn id(&self) -> &'static str {
        "birds_eye_ipm"
    }

    fn process(&mut self, input: FrameView<'_>) -> Result<ProcessedFrame, EdgeVisionError> {
        let _ = (self.backend, &self.calibration);
        if input.data.is_empty() {
            return Err(EdgeVisionError::Config(
                "empty frame buffer (stub requires non-empty slice)".into(),
            ));
        }
        Err(EdgeVisionError::NotImplemented(
            "IPM / homography remap → kornia-rs GPU kernel (CubeCL or CUDA)",
        ))
    }
}
