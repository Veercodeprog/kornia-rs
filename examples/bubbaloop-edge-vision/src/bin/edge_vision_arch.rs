//! Prints architecture summary for proposals (`cargo run -p bubbaloop-edge-vision --bin edge-vision-arch`).

use bubbaloop_edge_vision::bubbaloop::NodeSurface;
use bubbaloop_edge_vision::config::BubbaloopNamespace;
use bubbaloop_edge_vision::{AppMode, ComputeBackend, EdgeVisionHub, FrameView};

fn main() {
    let ns = BubbaloopNamespace::from_env();
    let surface = NodeSurface::example_edge_vision_processor(&ns);

    println!("=== bubbaloop-edge-vision (architecture demo) ===\n");
    println!("Zenoh namespace (from env or defaults):");
    println!("  scope       = {}", ns.scope);
    println!("  machine_id  = {}", ns.machine_id);
    println!("\nExample camera subscription key:");
    println!("  {}", ns.camera_compressed_topic("front"));
    println!("\nExample manifest (JSON):\n{}", pretty(&surface).unwrap());

    for mode in [
        AppMode::BirdsEye,
        AppMode::Panorama,
        AppMode::Surround,
    ] {
        let mut hub = EdgeVisionHub::new(mode, ComputeBackend::GpuCuda);
        let dummy = vec![0u8; 16];
        let frame = FrameView {
            width: 4,
            height: 4,
            channels: 1,
            data: &dummy,
        };
        let outcome = hub.process(frame);
        println!("\nMode {:?} → {:?}", mode, outcome.map_err(|e| e.to_string()));
    }
}

fn pretty<T: serde::Serialize>(v: &T) -> Result<String, serde_json::Error> {
    serde_json::to_string_pretty(v)
}
