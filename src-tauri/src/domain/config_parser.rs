use std::path::PathBuf;

use serde_json::Value;

use super::errors::AppError;
use super::models::{McpServer, ProviderId};

/// Result of parsing a provider config file.
pub struct ParseResult {
    /// Normalized MCP servers extracted from the config.
    pub servers: Vec<McpServer>,
    /// Raw metadata for untouched keys (preserved through write-back).
    pub preserved_raw: Value,
}

/// Request to write servers back to a config file.
pub struct WriteBackRequest {
    /// The modified server list to persist.
    pub servers: Vec<McpServer>,
    /// Original untouched keys to preserve.
    pub preserved_raw: Value,
}

/// Trait for parsing and writing provider config files.
///
/// Each provider (OpenCode, Claude, Codex) implements this trait
/// to handle its specific config format while presenting a unified interface.
pub trait ConfigParser: Send + Sync {
    /// Detect if config file exists at the expected OS path.
    /// Returns the path if found, None otherwise.
    fn detect(&self) -> Option<PathBuf>;

    /// Parse config file into normalized McpServer list.
    /// Returns servers plus raw metadata for write-back fidelity.
    fn parse(&self, path: &PathBuf) -> Result<ParseResult, AppError>;

    /// Write servers back to config file, preserving untouched keys.
    fn write_back(&self, path: &PathBuf, request: WriteBackRequest) -> Result<(), AppError>;

    /// Provider this parser handles.
    fn provider_id(&self) -> ProviderId;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::models::{McpServer, Transport};
    use serde_json::json;

    #[test]
    fn parse_result_preserves_metadata() {
        let servers = vec![McpServer {
            name: "test".into(),
            command: "node".into(),
            args: vec!["server.js".into()],
            env: Default::default(),
            transport: Transport::Stdio,
            enabled: true,
        }];

        let preserved = json!({"other_key": "preserved"});

        let result = ParseResult {
            servers,
            preserved_raw: preserved.clone(),
        };

        assert_eq!(result.servers.len(), 1);
        assert_eq!(result.servers[0].name, "test");
        assert_eq!(result.preserved_raw, preserved);
    }

    #[test]
    fn write_back_request_holds_data() {
        let servers = vec![];
        let preserved = json!({});

        let req = WriteBackRequest {
            servers,
            preserved_raw: preserved.clone(),
        };

        assert!(req.servers.is_empty());
        assert_eq!(req.preserved_raw, preserved);
    }
}
