use std::path::PathBuf;

use serde::Serialize;

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("Config file not found: {path}")]
    ConfigNotFound { path: PathBuf },

    #[error("Config parse error in {path}: {message}")]
    ConfigParse { path: PathBuf, message: String },

    #[error("Config file not writable: {path}")]
    ConfigNotWritable { path: PathBuf },

    #[error("MCP validation error: {0}")]
    Validation(String),

    #[error("Process spawn timeout for '{name}' after {timeout_secs}s")]
    SpawnTimeout { name: String, timeout_secs: u64 },

    #[error("Command not found: {command}")]
    CommandNotFound { command: String },

    #[error("MCP server '{name}' unresponsive after {failures} failed pings")]
    ServerUnresponsive { name: String, failures: u32 },

    #[error("API key not configured for {provider}")]
    ApiKeyMissing { provider: String },

    #[error("Invalid API key for {provider}")]
    ApiKeyInvalid { provider: String },

    #[error("Network error: {0}")]
    Network(String),

    #[error("Keychain error: {0}")]
    Keychain(String),

    #[error("Database error: {0}")]
    Database(String),

    #[error("Settings error: {0}")]
    Settings(String),
}

// Tauri commands return Result<T, AppError>.
// AppError implements Into<InvokeError> via serde serialization.
impl Serialize for AppError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn error_display_messages() {
        let err = AppError::ConfigNotFound {
            path: PathBuf::from("/tmp/test.json"),
        };
        assert_eq!(err.to_string(), "Config file not found: /tmp/test.json");

        let err = AppError::Validation("name is required".into());
        assert_eq!(err.to_string(), "MCP validation error: name is required");

        let err = AppError::SpawnTimeout {
            name: "my-server".into(),
            timeout_secs: 10,
        };
        assert_eq!(
            err.to_string(),
            "Process spawn timeout for 'my-server' after 10s"
        );
    }

    #[test]
    fn error_serialize_to_string() {
        let err = AppError::ApiKeyMissing {
            provider: "openai".into(),
        };
        let json = serde_json::to_string(&err).unwrap();
        assert_eq!(json, "\"API key not configured for openai\"");
    }
}
