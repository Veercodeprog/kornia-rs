# Architecture: bubbaloop-edge-vision

## Goals

- **Live** processing on **GPU-equipped** edge devices (e.g. Jetson Orin).
- **Reusable** vision ops in **kornia-rs** (not locked inside one binary).
- **Orchestration** via **Bubbaloop**: camera nodes publish to **Zenoh**; this logic runs as a **separate node** (or merged into a custom capture+process node later).

## High-level data flow

```text
┌─────────────────────┐     Zenoh (high rate)      ┌──────────────────────────┐
│  rtsp-camera /     │  ───────────────────────►  │  edge-vision node        │
│  v4l2-camera node   │   compressed / raw topics  │  (this crate → future)   │
│  (official repo)    │                            │  GPU: CubeCL / CUDA / …  │
└─────────────────────┘                            └────────────┬─────────────┘
                                                                │
                    Zenoh: processed output (e.g. BEV tensor,  │
                    stitched frame, surround composite)         ▼
                                                     ┌─────────────────────┐
                                                     │ Dashboard / agents  │
                                                     │ (MCP = control only)│
                                                     └─────────────────────┘
```

**MCP** (in Bubbaloop) is for **discovery, commands, config** — not for streaming video at 30 FPS. Streaming stays on **Zenoh**, per Bubbaloop architecture docs.

## Three pipeline modes (proposal scope)

| Mode | Role | Typical inputs | Output (conceptual) |
|------|------|----------------|---------------------|
| **Bird's-eye (IPM)** | Inverse perspective mapping for driving / robotics | Single forward camera + **homography / intrinsics** (calibration) | BEV image or tensor on ground plane |
| **Live panorama** | Stitch overlapping views over time or from multiple streams | 2+ streams or 1 moving camera + **motion / feature** links | Wide FOV stitched frame |
| **Multi-camera surround** | Around-vehicle or room coverage | 4+ calibrated cameras + **extrinsics** | Composite “top-down” or tiled surround view |

This repository encodes these as **separate modules** with a shared `VisionPipeline` trait so you can deliver **one** full vertical in GSoC while keeping hooks for the others.

## Compute backends (future)

| Backend | When to use |
|---------|-------------|
| **CPU** | Baseline, correctness tests, CI |
| **CubeCL** | Portable Rust-side GPU kernels ([CubeCL](https://github.com/tracel-ai/cubecl)) |
| **CUDA** | Maximum performance on NVIDIA Jetson / discrete GPUs |

The crate `ComputeBackend` enum in `lib.rs` is a placeholder for selecting backends at runtime or compile time.

## kornia-rs integration (intended)

- **Decode / resize / color** may stay in **kornia-io** / **kornia-imgproc** (CPU or future GPU).
- **Geometric warps** (homography, remap) and **tensor-friendly** buffers belong in **kornia-tensor** + **kornia-imgproc** (and new GPU modules as accepted upstream).
- This example crate should eventually call **public kornia APIs** only — no duplicated CV math here.

## Benchmarks (expected for full project)

- Same resolution and pipeline: **CPU path** vs **GPU path** (latency ms/frame, FPS, peak memory).
- Document hardware (SoC, JetPack / driver versions).
