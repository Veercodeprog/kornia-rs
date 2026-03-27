//! Architecture skeleton for **Bubbaloop** + **kornia-rs** edge vision (proposal / GSoC).
//!
//! Implements **no** GPU kernels and **no** Zenoh I/O here — only types, topic conventions,
//! and stub pipelines for **bird's-eye**, **panorama**, and **multi-camera surround**.
//!
//! See `docs/ARCHITECTURE.md` and `docs/BUBBALOOP_INTEGRATION.md`.

#![forbid(unsafe_code)]

pub mod bubbaloop;
pub mod config;
pub mod pipelines;

use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Which high-level demo mode the hub runs (select **one** for a focused deliverable).
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AppMode {
    /// Inverse perspective / bird's-eye from a forward camera + calibration.
    BirdsEye,
    /// Rolling or multi-frame panorama stitching.
    Panorama,
    /// Multiple fixed cameras → surround-style composite.
    Surround,
}

/// Placeholder for future backend selection (CubeCL, CUDA, etc.).
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ComputeBackend {
    #[default]
    Cpu,
    GpuCubeCl,
    GpuCuda,
}

/// Single decoded frame placeholder (bytes + dimensions). Replace with `kornia::image::Image` later.
#[derive(Clone, Debug)]
pub struct FrameView<'a> {
    pub width: u32,
    pub height: u32,
    pub channels: u8,
    pub data: &'a [u8],
}

/// Opaque output handle (e.g. future tensor id or encoded bitstream).
#[derive(Clone, Debug, Default)]
pub struct ProcessedFrame {
    pub width: u32,
    pub height: u32,
    pub note: String,
}

/// Errors for stub processing (expand when wiring Zenoh / kornia-rs).
#[derive(Debug, Error)]
pub enum EdgeVisionError {
    #[error("pipeline not implemented: {0}")]
    NotImplemented(&'static str),

    #[error("invalid configuration: {0}")]
    Config(String),
}

/// Common interface for the three proposal pipelines.
pub trait VisionPipeline {
    /// Human-readable id for logs and manifests.
    fn id(&self) -> &'static str;

    /// Stub: would decode, upload to GPU, run kornia-rs ops, download / encode.
    fn process(&mut self, input: FrameView<'_>) -> Result<ProcessedFrame, EdgeVisionError>;
}

/// Central switch: picks pipeline by [`AppMode`].
#[derive(Debug)]
pub struct EdgeVisionHub {
    mode: AppMode,
    backend: ComputeBackend,
    birdseye: pipelines::birdseye::BirdsEyePipeline,
    panorama: pipelines::panorama::PanoramaPipeline,
    surround: pipelines::surround::SurroundPipeline,
}

impl EdgeVisionHub {
    /// Builds a hub with the given mode and compute backend (GPU paths are stubs).
    pub fn new(mode: AppMode, backend: ComputeBackend) -> Self {
        Self {
            mode,
            backend,
            birdseye: pipelines::birdseye::BirdsEyePipeline::new(backend),
            panorama: pipelines::panorama::PanoramaPipeline::new(backend),
            surround: pipelines::surround::SurroundPipeline::new(backend),
        }
    }

    /// Returns the configured application mode.
    pub fn mode(&self) -> AppMode {
        self.mode
    }

    /// Returns the selected compute backend placeholder.
    pub fn backend(&self) -> ComputeBackend {
        self.backend
    }

    /// Dispatches to the active pipeline stub.
    pub fn process(&mut self, input: FrameView<'_>) -> Result<ProcessedFrame, EdgeVisionError> {
        match self.mode {
            AppMode::BirdsEye => VisionPipeline::process(&mut self.birdseye, input),
            AppMode::Panorama => VisionPipeline::process(&mut self.panorama, input),
            AppMode::Surround => VisionPipeline::process(&mut self.surround, input),
        }
    }
}
