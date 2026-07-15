use serde::{Deserialize, Serialize};

/// Runtime state of a spawned MCP server.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ServerState {
    Running,
    Stopped,
    Error,
    Unresponsive,
}

/// Telemetry metrics for a single MCP server at a point in time.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TelemetryMetrics {
    pub state: ServerState,
    pub latency_ms: Option<f64>,
    pub errors: u32,
    pub uptime_s: u64,
    pub cpu_pct: Option<f64>,
    pub mem_mb: Option<f64>,
}

/// A single telemetry snapshot for time-series storage.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricsSnapshot {
    pub mcp_name: String,
    pub timestamp: String,
    pub state: ServerState,
    pub latency_ms: Option<f64>,
    pub errors: u32,
    pub uptime_s: u64,
    pub cpu_pct: Option<f64>,
    pub mem_mb: Option<f64>,
}

/// Result of a heartbeat ping to an MCP server.
#[derive(Debug, Clone)]
pub struct HeartbeatResult {
    pub latency_ms: f64,
    pub success: bool,
}

impl Default for TelemetryMetrics {
    fn default() -> Self {
        Self {
            state: ServerState::Stopped,
            latency_ms: None,
            errors: 0,
            uptime_s: 0,
            cpu_pct: None,
            mem_mb: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn server_state_serializes_lowercase() {
        let running = ServerState::Running;
        let json = serde_json::to_string(&running).unwrap();
        assert_eq!(json, "\"running\"");

        let stopped = ServerState::Stopped;
        let json = serde_json::to_string(&stopped).unwrap();
        assert_eq!(json, "\"stopped\"");
    }

    #[test]
    fn telemetry_metrics_default() {
        let m = TelemetryMetrics::default();
        assert_eq!(m.state, ServerState::Stopped);
        assert!(m.latency_ms.is_none());
        assert_eq!(m.errors, 0);
        assert_eq!(m.uptime_s, 0);
    }

    #[test]
    fn metrics_snapshot_roundtrip() {
        let snapshot = MetricsSnapshot {
            mcp_name: "test-server".into(),
            timestamp: "2026-07-13T20:00:00Z".into(),
            state: ServerState::Running,
            latency_ms: Some(42.5),
            errors: 0,
            uptime_s: 300,
            cpu_pct: Some(1.2),
            mem_mb: Some(25.6),
        };

        let json = serde_json::to_string(&snapshot).unwrap();
        let deserialized: MetricsSnapshot = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.mcp_name, "test-server");
        assert_eq!(deserialized.state, ServerState::Running);
        assert_eq!(deserialized.latency_ms, Some(42.5));
        assert_eq!(deserialized.uptime_s, 300);
    }
}
