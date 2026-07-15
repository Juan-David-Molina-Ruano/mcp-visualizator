use std::collections::HashMap;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

/// Supported MCP transport (stdio only for MVP).
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Transport {
    #[default]
    Stdio,
}

/// Normalized MCP server representation across all providers.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpServer {
    pub name: String,
    pub command: String,
    pub args: Vec<String>,
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub env: HashMap<String, String>,
    #[serde(default)]
    pub transport: Transport,
    #[serde(default = "default_true")]
    pub enabled: bool,
}

fn default_true() -> bool {
    true
}

/// Provider identifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ProviderId {
    OpenCode,
    Claude,
    Codex,
}

impl ProviderId {
    pub fn as_str(&self) -> &'static str {
        match self {
            ProviderId::OpenCode => "opencode",
            ProviderId::Claude => "claude",
            ProviderId::Codex => "codex",
        }
    }
}

impl std::fmt::Display for ProviderId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// A detected provider with its config path and servers.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Provider {
    pub id: ProviderId,
    pub name: String,
    pub config_path: PathBuf,
    pub detected: bool,
    pub servers: Vec<McpServer>,
}

/// A curated catalog entry (bundled in binary).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CatalogEntry {
    pub name: String,
    pub description: String,
    pub command: String,
    pub args: Vec<String>,
    #[serde(default)]
    pub env: HashMap<String, String>,
    pub homepage: String,
}

/// Application settings persisted in SQLite.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSettings {
    pub polling_interval_ms: u64,
    pub spawn_timeout_secs: u64,
    pub ping_timeout_secs: u64,
    pub consecutive_failures_threshold: u32,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            polling_interval_ms: 30_000,
            spawn_timeout_secs: 10,
            ping_timeout_secs: 5,
            consecutive_failures_threshold: 3,
        }
    }
}
