//! Bubbaloop-facing surface: what a real **node** would expose (manifest / Zenoh / MCP split).
//!
//! High-rate data → **Zenoh**. Control → **MCP** + node **command** queryable.

use serde::{Deserialize, Serialize};

use crate::AppMode;

/// JSON-friendly description for a future `manifest` queryable.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NodeSurface {
    pub node_name: String,
    pub description: String,
    pub subscribes: Vec<String>,
    pub publishes: Vec<String>,
    pub commands: Vec<String>,
    pub supported_modes: Vec<AppMode>,
}

impl NodeSurface {
    /// Example manifest for an `edge-vision-processor` node using default topic names.
    pub fn example_edge_vision_processor(ns: &crate::config::BubbaloopNamespace) -> Self {
        Self {
            node_name: "edge-vision-processor".to_string(),
            description: "GPU-oriented vision: bird's-eye, panorama, or surround (stub)".to_string(),
            subscribes: vec![format!(
                "bubbaloop/{}/{}/camera/*/compressed",
                ns.scope, ns.machine_id
            )],
            publishes: vec![
                ns.vision_bev_topic(),
                ns.vision_panorama_topic(),
                ns.vision_surround_topic(),
            ],
            commands: vec![
                "set_mode".to_string(),
                "reload_calibration".to_string(),
                "set_backend".to_string(),
            ],
            supported_modes: vec![AppMode::BirdsEye, AppMode::Panorama, AppMode::Surround],
        }
    }
}
