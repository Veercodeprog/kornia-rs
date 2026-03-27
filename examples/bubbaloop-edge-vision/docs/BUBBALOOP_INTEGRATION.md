# Bubbaloop integration guide

How this **architecture skeleton** connects to **[Bubbaloop](https://github.com/kornia/bubbaloop)** and **[bubbaloop-nodes-official](https://github.com/kornia/bubbaloop-nodes-official)**.

## 1. What Bubbaloop provides

- **Daemon**: node lifecycle (`install`, `build`, `start`, `stop`), health, MCP server.
- **Zenoh**: pub/sub for **sensor data** (video, telemetry). Topic prefix:
  `bubbaloop/{scope}/{machine_id}/...`
- **Environment** (injected for nodes): `BUBBALOOP_SCOPE`, `BUBBALOOP_MACHINE_ID`, `BUBBALOOP_ZENOH_ENDPOINT`.

## 2. Camera input (reuse official nodes)

From **bubbaloop-nodes-official**:

- **`rtsp-camera`**: publishes to  
  `bubbaloop/{scope}/{machine}/camera/{instance}/compressed`  
  (H.264 / GStreamer pipeline; see that repo’s `configs/` examples).
- **`v4l2` driver** (YAML skills in Bubbaloop): maps to **v4l2-camera** marketplace node for USB/CSI cameras.

**Your processing node** should **subscribe** to those topics (or to a raw topic if you add one), not re-implement RTSP unless needed.

## 3. Turning this crate into a Bubbaloop node

Recommended path for a proposal / implementation:

1. **Scaffold** with Bubbaloop CLI (from their docs):
   ```bash
   bubbaloop node init edge-vision-processor --node-type rust -o ./edge-vision-processor
   ```
2. **Add dependency** on this logic (copy `src/` into the node crate or publish as a small lib crate).
3. **Wire Zenoh** in the node’s `run` loop:
   - `subscriber` on `camera/*/compressed` (or explicit topic from config).
   - `publisher` on e.g. `vision/bev/output` or `vision/panorama/output` (choose one namespace and document it in the node **manifest**).
4. **Protobuf**: follow **bubbaloop-schemas** from the Bubbaloop repo — build `FileDescriptorSet`, expose `{node}/schema` queryable (see Bubbaloop ARCHITECTURE.md).
5. **Manifest / health / config / command**: implement the **node contract** so MCP tools (`list_nodes`, `send_command`, …) work.

## 4. MCP vs Zenoh (reminder)

- **MCP**: “start processing”, “switch mode: birdseye|panorama|surround”, “set calibration path”.
- **Zenoh**: actual **frame** and **tensor** streams.

## 5. Dashboard / agents

- **Dashboard** can subscribe to your **output** Zenoh topic (same pattern as camera panel).
- **Agents** discover your node via manifest and can call **commands**; they do not need raw frame bytes in the LLM path for the core demo.

## 6. References

- Bubbaloop README: [github.com/kornia/bubbaloop](https://github.com/kornia/bubbaloop)
- Official nodes: [github.com/kornia/bubbaloop-nodes-official](https://github.com/kornia/bubbaloop-nodes-official)
- kornia-rs: [github.com/kornia/kornia-rs](https://github.com/kornia/kornia-rs)
