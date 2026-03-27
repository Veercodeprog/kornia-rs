# bubbaloop-edge-vision

**Edge vision architecture for Bubbaloop + kornia-rs** — bird's-eye (IPM), live panorama, and multi-camera surround, designed for **GPU** on devices like Jetson Orin.

| | |
|--|--|
| **Status** | **Architecture / proposal skeleton** — compiles and documents the design; **no** live camera, Zenoh, GPU kernels, or Bubbaloop daemon wiring yet. |
| **Repo** | Lives under **kornia-rs** as `examples/bubbaloop-edge-vision`. |
| **Last intent** | Describe *how* the full app will work and *how* it plugs into Bubbaloop; implement processing in **kornia-rs** and a real **Bubbaloop node** later. |

---

## Table of contents

1. [What this is (in plain words)](#what-this-is-in-plain-words)
2. [What works today vs what comes later](#what-works-today-vs-what-comes-later)
3. [How it fits with other projects](#how-it-fits-with-other-projects)
4. [The three vision modes](#the-three-vision-modes)
5. [How data is supposed to flow](#how-data-is-supposed-to-flow)
6. [Project layout](#project-layout)
7. [Quick start](#quick-start)
8. [Configuration example](#configuration-example)
9. [Roadmap (full project)](#roadmap-full-project)
10. [More documentation](#more-documentation)
11. [Useful links](#useful-links)

---

## What this is (in plain words)

This crate is a **blueprint** for a program that would:

1. **Receive** live camera video from **Bubbaloop** (over **Zenoh**, same style as official camera nodes).
2. **Process** it with **computer vision** (eventually on the **GPU**, using code shared through **kornia-rs**).
3. **Send** the result back on Zenoh (for a dashboard, robot stack, or AI agent metadata).

It is **not** a finished product. It **is** a good base for a **proposal**, **design review**, or **incremental build**: clear modules, topic names, and docs so the next steps are obvious.

---

## What works today vs what comes later

| Area | Today | Later (full project) |
|------|--------|----------------------|
| **Rust crate** | Builds; types, traits, topic helpers, demo binary | Same, plus real processing |
| **Bird's-eye / panorama / surround** | **Stub** pipelines (`NotImplemented` errors with hints) | Real algorithms + **kornia-rs** GPU (e.g. CubeCL / CUDA) |
| **Bubbaloop** | Documented in `docs/`; `NodeSurface` example manifest | Real **node** with Zenoh + `bubbaloop-schemas` |
| **Cameras** | Topic names match **bubbaloop-nodes-official** style | Subscribe to `rtsp-camera` / `v4l2` outputs |
| **MCP / agents** | Explained in docs only | Commands like `set_mode` on the real node |
| **Benchmarks & demo video** | Not here | CPU vs GPU + hardware recording |

*Tip:* When you change what is implemented, update the **Status** table at the top so readers always see the **current** picture.

---

## How it fits with other projects

| Project | Role |
|---------|------|
| **[Bubbaloop](https://github.com/kornia/bubbaloop)** | Runs the **daemon**, **Zenoh**, **MCP**, and **node** lifecycle (`install`, `start`, …). Your future processing app is one **node** (or uses nodes). |
| **[bubbaloop-nodes-official](https://github.com/kornia/bubbaloop-nodes-official)** | Provides **rtsp-camera** (and similar) nodes that **publish** live video to Zenoh, e.g. under `…/camera/{name}/compressed`. You **reuse** that for input. |
| **[kornia-rs](https://github.com/kornia/kornia-rs)** | Where **reusable** image/tensor and (future) **GPU** code should live, then get called from your node — not only hidden in a private binary. |
| **This crate** | **Design + stubs** inside kornia-rs `examples/` until you promote it to a standalone node repo or merge into official nodes. |

**Simple rule:** **Zenoh** = fast video/data. **MCP** = control and discovery (not 30 FPS frame transport).

---

## The three vision modes

All three exist as **modules** and share one **`VisionPipeline`** trait. For a **single** deliverable (e.g. GSoC), you usually **finish one mode first** and keep the others as extensions.

| Mode | Idea | Typical inputs |
|------|------|----------------|
| **Bird's-eye (`BirdsEye`)** | Map a forward camera to a **top-down** “ground” view (IPM / homography). | One camera stream + **calibration** (homography or intrinsics). |
| **Panorama (`Panorama`)** | **Stitch** views into a wider image (over time or multiple cameras). | Overlapping frames + alignment (features / motion). |
| **Surround (`Surround`)** | **Combine** several fixed cameras (e.g. front/rear/left/right). | Multiple Zenoh camera topics + **extrinsics** / layout. |

The **`EdgeVisionHub`** picks **one** `AppMode` at a time and calls the matching pipeline.

---

## How data is supposed to flow

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
  Dashboard / robot / agent (metadata via MCP, not raw FPS through MCP)
```

Environment variables (when running under Bubbaloop) typically include **`BUBBALOOP_SCOPE`**, **`BUBBALOOP_MACHINE_ID`**, and Zenoh endpoint settings — see [Bubbaloop docs](https://github.com/kornia/bubbaloop).

---

## Project layout

| Path | What it is |
|------|------------|
| `src/lib.rs` | `EdgeVisionHub`, `AppMode`, `ComputeBackend`, `VisionPipeline`, errors |
| `src/config.rs` | Topic string builders; reads `BUBBALOOP_*` env vars |
| `src/bubbaloop.rs` | Example **`NodeSurface`** (manifest-style JSON for a future node) |
| `src/pipelines/birdseye.rs` | Bird's-eye / IPM stub |
| `src/pipelines/panorama.rs` | Panorama stub |
| `src/pipelines/surround.rs` | Multi-camera surround stub |
| `src/bin/edge_vision_arch.rs` | Small demo: prints topics + manifest + stub errors |
| `configs/proposal.example.toml` | Example **future** runtime config (not loaded by code yet) |
| `docs/ARCHITECTURE.md` | Deeper architecture, backends, benchmarks |
| `docs/BUBBALOOP_INTEGRATION.md` | Step-by-step: node init, Zenoh, schemas, MCP |

---

## Quick start

From the **kornia-rs** workspace root:

```bash
# Check that the example compiles
cargo check -p bubbaloop-edge-vision

# Print example Zenoh keys, manifest JSON, and stub pipeline messages
cargo run -p bubbaloop-edge-vision --bin edge-vision-arch
```

Optional — match your machine name in the demo output:

```bash
export BUBBALOOP_SCOPE=local
export BUBBALOOP_MACHINE_ID=jetson_orin_01
cargo run -p bubbaloop-edge-vision --bin edge-vision-arch
```

**Dependencies (this crate only):** `serde`, `serde_json`, `thiserror`. No Zenoh, no Bubbaloop SDK, no kornia crates yet — on purpose, so the skeleton stays easy to build anywhere.

---

## Configuration example

See **`configs/proposal.example.toml`** for a **human-readable** sketch of how a real node might choose `mode`, `backend`, camera instance, and calibration paths. The library does **not** load this file yet; copy the idea into your future Bubbaloop node config.

---

## Roadmap (full project)

1. **kornia-rs** — Add the actual image/tensor ops and **GPU** paths (e.g. [CubeCL](https://github.com/tracel-ai/cubecl) or CUDA), with **CPU vs GPU benchmarks**.
2. **Bubbaloop node** — `bubbaloop node init …`, Zenoh subscriber/publisher, protobuf from **bubbaloop-schemas**, manifest/health/config/command.
3. **Hardware** — Run on **Jetson** (or similar): live input from **rtsp-camera** or **v4l2**, demo **video**, documented FPS and latency.

---

## More documentation

| Document | Use it when you need… |
|----------|------------------------|
| [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md) | Diagrams, MCP vs Zenoh, backend choices, benchmark expectations |
| [docs/BUBBALOOP_INTEGRATION.md](docs/BUBBALOOP_INTEGRATION.md) | Concrete Bubbaloop + official nodes + node contract steps |

---

## Useful links

- [Bubbaloop](https://github.com/kornia/bubbaloop) — daemon, Zenoh, MCP, CLI
- [bubbaloop-nodes-official](https://github.com/kornia/bubbaloop-nodes-official) — **rtsp-camera**, other sensor nodes
- [kornia-rs](https://github.com/kornia/kornia-rs) — library and workspace that hosts this example
- [CubeCL](https://github.com/tracel-ai/cubecl) — optional Rust GPU compute stack (mentioned in many GSoC-style specs)

---

## License

Same as the **kornia-rs** workspace: **Apache-2.0** (see repository `LICENSE`).
