# bubbaloop-edge-vision

Edge vision layout for **Bubbaloop** and **kornia-rs**: bird's-eye (IPM), live panorama, and multi-camera surround, aimed at **GPU** execution on hardware such as Jetson Orin.

| | |
|--|--|
| **Status** | Architecture skeleton — builds, documents topics and node shape; no live camera, Zenoh I/O, GPU kernels, or Bubbaloop daemon integration yet. |
| **Location** | `examples/bubbaloop-edge-vision` in the kornia-rs workspace. |
| **Scope** | Clarify how a full app would connect to Bubbaloop; implement vision in **kornia-rs** and ship a real **Bubbaloop node** in a follow-up. |

---

## Contents

1. [Overview](#overview)
2. [Current state vs planned work](#current-state-vs-planned-work)
3. [Related repositories](#related-repositories)
4. [Vision modes](#vision-modes)
5. [Data flow](#data-flow)
6. [Repository layout](#repository-layout)
7. [Quick start](#quick-start)
8. [Configuration](#configuration)
9. [Roadmap](#roadmap)
10. [Further reading](#further-reading)
11. [Links](#links)

---

## Overview

This crate sketches an application that would:

1. Ingest live camera streams from Bubbaloop over **Zenoh**, using the same topic style as official camera nodes.
2. Run **computer vision** on those frames, with heavy work intended to land in **kornia-rs** and eventually on the **GPU**.
3. Publish results back on Zenoh for dashboards, robotics stacks, or agent-facing metadata.

It is not a complete application. It is a structured starting point for design discussion or incremental implementation: module boundaries, topic naming, and integration notes are fixed early so later work stays consistent.

---

## Current state vs planned work

| Area | Now | Planned |
|------|-----|---------|
| Crate | Compiles; types, traits, topic helpers, demo binary | Same surface, plus real processing |
| Bird's-eye / panorama / surround | Stub pipelines (errors document intended kornia-rs GPU work) | Algorithms + GPU paths in kornia-rs (e.g. CubeCL or CUDA) |
| Bubbaloop | Described under `docs/`; example `NodeSurface` manifest | Node with Zenoh + `bubbaloop-schemas` |
| Cameras | Topic layout aligned with bubbaloop-nodes-official | Subscribe to `rtsp-camera` / `v4l2` publishers |
| MCP / agents | Documentation only | Command interface on the real node (e.g. `set_mode`) |
| Benchmarks, demo video | Out of scope here | CPU vs GPU metrics and on-device recording |

Keep the **Status** table at the top in sync when capabilities change.

---

## Related repositories

| Project | Role |
|---------|------|
| [Bubbaloop](https://github.com/kornia/bubbaloop) | Daemon, Zenoh, MCP, node lifecycle. The processor would run as a node (or alongside existing nodes). |
| [bubbaloop-nodes-official](https://github.com/kornia/bubbaloop-nodes-official) | `rtsp-camera` and related nodes publishing to Zenoh (e.g. `…/camera/{name}/compressed`). Use them for input instead of reimplementing capture when possible. |
| [kornia-rs](https://github.com/kornia/kornia-rs) | Shared image, tensor, and (future) GPU code; contributions should be usable beyond a single binary. |
| This directory | Design and stubs under `examples/` until promoted to a standalone node or merged elsewhere. |

Zenoh carries high-rate sensor and video data. MCP is for control, discovery, and configuration — not for pushing full frame streams at video rates.

---

## Vision modes

The three modes are separate modules behind a shared `VisionPipeline` trait. A single deliverable can focus on one mode first; the others remain as extensions.

| Mode | Purpose | Typical inputs |
|------|---------|----------------|
| **Bird's-eye (`BirdsEye`)** | Inverse perspective / homography from a forward camera to a ground-plane or top-down view. | One stream plus calibration (homography or intrinsics). |
| **Panorama (`Panorama`)** | Stitch overlapping views over time or from multiple cameras. | Overlapping frames and alignment (features or motion). |
| **Surround (`Surround`)** | Fuse several fixed cameras (e.g. vehicle or room coverage). | Multiple Zenoh camera topics plus extrinsics / layout. |

`EdgeVisionHub` selects one `AppMode` and delegates to the matching pipeline.

---

## Data flow

```text
  Official camera node (e.g. rtsp-camera)
           │
           │  Zenoh publish
           ▼
  bubbaloop/{scope}/{machine}/camera/{instance}/compressed
           │
           │  subscribe (future node)
           ▼
  edge-vision processor  ──►  kornia-rs (CPU/GPU ops)
           │
           │  Zenoh publish
           ▼
  …/vision/bev/frame  or  …/panorama/frame  or  …/surround/frame
           │
           ▼
  Dashboard / robot stack (MCP for control; not raw video through MCP)
```

Under Bubbaloop, nodes commonly receive `BUBBALOOP_SCOPE`, `BUBBALOOP_MACHINE_ID`, and Zenoh settings from the environment. See the [Bubbaloop repository](https://github.com/kornia/bubbaloop) for current behavior.

---

## Repository layout

| Path | Description |
|------|-------------|
| `src/lib.rs` | `EdgeVisionHub`, `AppMode`, `ComputeBackend`, `VisionPipeline`, errors |
| `src/config.rs` | Topic builders; `BUBBALOOP_*` environment variables |
| `src/bubbaloop.rs` | Example `NodeSurface` (manifest-oriented JSON) |
| `src/pipelines/birdseye.rs` | Bird's-eye / IPM stub |
| `src/pipelines/panorama.rs` | Panorama stub |
| `src/pipelines/surround.rs` | Surround stub |
| `src/bin/edge_vision_arch.rs` | Demo: prints sample keys, manifest JSON, stub errors |
| `configs/proposal.example.toml` | Sample runtime config (not loaded by this crate yet) |
| `docs/ARCHITECTURE.md` | Architecture detail, backends, benchmarks |
| `docs/BUBBALOOP_INTEGRATION.md` | Node init, Zenoh, schemas, MCP |

---

## Quick start

From the kornia-rs workspace root:

```bash
cargo check -p bubbaloop-edge-vision
cargo run -p bubbaloop-edge-vision --bin edge-vision-arch
```

Optional environment for realistic topic prefixes in the demo:

```bash
export BUBBALOOP_SCOPE=local
export BUBBALOOP_MACHINE_ID=jetson_orin_01
cargo run -p bubbaloop-edge-vision --bin edge-vision-arch
```

Dependencies here are only `serde`, `serde_json`, and `thiserror`. Zenoh, Bubbaloop SDK, and kornia crates are intentionally omitted so the skeleton builds in minimal environments.

---

## Configuration

`configs/proposal.example.toml` shows how a future node might set `mode`, `backend`, camera instance, and calibration paths. This library does not parse that file; reuse the structure in a real Bubbaloop node configuration when wiring the daemon.

---

## Roadmap

1. **kornia-rs** — Implement the required image/tensor operations and GPU backends ([CubeCL](https://github.com/tracel-ai/cubecl), CUDA, or both), with CPU vs GPU benchmarks.
2. **Bubbaloop node** — Scaffold with `bubbaloop node init`, add Zenoh subscribers/publishers and protobuf from `bubbaloop-schemas`, implement manifest/health/config/command.
3. **Hardware** — Deploy on Jetson-class hardware with `rtsp-camera` or `v4l2` input, record a short demo, and document throughput and latency.

---

## Further reading

| Document | Contents |
|----------|----------|
| [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md) | Diagrams, transport split, backend notes, benchmark expectations |
| [docs/BUBBALOOP_INTEGRATION.md](docs/BUBBALOOP_INTEGRATION.md) | Bubbaloop node contract, official nodes, MCP usage |

---

## Links

- [Bubbaloop](https://github.com/kornia/bubbaloop)
- [bubbaloop-nodes-official](https://github.com/kornia/bubbaloop-nodes-official)
- [kornia-rs](https://github.com/kornia/kornia-rs)
- [CubeCL](https://github.com/tracel-ai/cubecl)

---

## License

Apache-2.0, same as the kornia-rs workspace (see repository `LICENSE`).
