//! **Multi-camera surround** pipeline (stub).
//!
//! Full version: N calibrated cameras → project to common plane or mesh → composite;
//! subscribe to **multiple** Zenoh camera topics in the real node.

use crate::{ComputeBackend, EdgeVisionError, FrameView, ProcessedFrame, VisionPipeline};

#[derive(Clone, Debug, Default)]
pub struct SurroundLayout {
    /// Camera instance names matching `bubbaloop/.../camera/{instance}/compressed`.
    pub camera_instances: Vec<String>,
}

#[derive(Debug)]
pub struct SurroundPipeline {
    pub backend: ComputeBackend,
    pub layout: SurroundLayout,
}

impl SurroundPipeline {
    pub fn new(backend: ComputeBackend) -> Self {
        Self {
            backend,
            layout: SurroundLayout {
                camera_instances: vec![
                    "front".into(),
                    "rear".into(),
                    "left".into(),
                    "right".into(),
                ],
            },
        }
    }
}

impl VisionPipeline for SurroundPipeline {
    fn id(&self) -> &'static str {
        "multi_camera_surround"
    }

    fn process(&mut self, input: FrameView<'_>) -> Result<ProcessedFrame, EdgeVisionError> {
        let _ = (&self.layout, self.backend);
        if input.data.is_empty() {
            return Err(EdgeVisionError::Config(
                "empty frame buffer (stub requires non-empty slice)".into(),
            ));
        }
        Err(EdgeVisionError::NotImplemented(
            "multi-subscriber fusion + extrinsic calibration + GPU composite (kornia-rs)",
        ))
    }
}
