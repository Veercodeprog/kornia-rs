//! Topic and environment conventions aligned with Bubbaloop + **bubbaloop-nodes-official**.

use serde::{Deserialize, Serialize};

/// Default Zenoh scope if `BUBBALOOP_SCOPE` is unset.
pub const DEFAULT_SCOPE: &str = "local";

/// Prefix for all Bubbaloop keys: `bubbaloop/{scope}/{machine_id}/...`.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BubbaloopNamespace {
    pub scope: String,
    pub machine_id: String,
}

impl BubbaloopNamespace {
    /// Reads from environment variables `BUBBALOOP_SCOPE` and `BUBBALOOP_MACHINE_ID`
    /// (fallback: `local` and `unknown_edge_device`).
    pub fn from_env() -> Self {
        let scope = std::env::var("BUBBALOOP_SCOPE").unwrap_or_else(|_| DEFAULT_SCOPE.to_string());
        let machine_id = std::env::var("BUBBALOOP_MACHINE_ID")
            .unwrap_or_else(|_| "unknown_edge_device".to_string());
        Self { scope, machine_id }
    }

    /// `rtsp-camera` style topic: `.../camera/{instance}/compressed`.
    pub fn camera_compressed_topic(&self, instance: &str) -> String {
        format!(
            "bubbaloop/{}/{}/camera/{}/compressed",
            self.scope, self.machine_id, instance
        )
    }

    /// Suggested publisher for bird's-eye / IPM output (protobuf payload TBD in real node).
    pub fn vision_bev_topic(&self) -> String {
        format!(
            "bubbaloop/{}/{}/vision/bev/frame",
            self.scope, self.machine_id
        )
    }

    /// Suggested publisher for panorama output.
    pub fn vision_panorama_topic(&self) -> String {
        format!(
            "bubbaloop/{}/{}/vision/panorama/frame",
            self.scope, self.machine_id
        )
    }

    /// Suggested publisher for surround composite.
    pub fn vision_surround_topic(&self) -> String {
        format!(
            "bubbaloop/{}/{}/vision/surround/frame",
            self.scope, self.machine_id
        )
    }
}
