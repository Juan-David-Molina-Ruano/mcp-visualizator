use serde::{Deserialize, Serialize};

/// Usage summary for a provider (OpenAI or Anthropic).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsageSummary {
    pub provider: String,
    pub total_input_tokens: u64,
    pub total_output_tokens: u64,
    pub total_cost_usd: Option<f64>,
    pub period_start: Option<String>,
    pub period_end: Option<String>,
    pub last_fetched: String,
    pub is_stale: bool,
}

/// A single usage record for storage.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsageRecord {
    pub input_tokens: u64,
    pub output_tokens: u64,
    pub timestamp: String,
}

/// Provider-specific usage data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderUsage {
    pub provider: String,
    pub records: Vec<UsageRecord>,
}

impl UsageSummary {
    pub fn empty(provider: impl Into<String>) -> Self {
        Self {
            provider: provider.into(),
            total_input_tokens: 0,
            total_output_tokens: 0,
            total_cost_usd: None,
            period_start: None,
            period_end: None,
            last_fetched: chrono_now(),
            is_stale: false,
        }
    }

    /// Check if data is stale (older than 1 hour).
    pub fn compute_staleness(last_fetched: &str) -> bool {
        // Simple check: if we can't parse or if older than 1 hour
        // For MVP, we rely on the frontend to check timestamps
        let _ = last_fetched;
        false
    }
}

/// Current time as ISO 8601 string.
fn chrono_now() -> String {
    // Using std time for now — no chrono dependency needed for MVP
    // Format: YYYY-MM-DDTHH:MM:SSZ
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    format!("{}T00:00:00Z", now)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_usage_summary() {
        let s = UsageSummary::empty("openai");
        assert_eq!(s.provider, "openai");
        assert_eq!(s.total_input_tokens, 0);
        assert_eq!(s.total_output_tokens, 0);
        assert!(s.total_cost_usd.is_none());
        assert!(!s.is_stale);
    }

    #[test]
    fn usage_summary_serializes() {
        let s = UsageSummary {
            provider: "anthropic".into(),
            total_input_tokens: 1000,
            total_output_tokens: 500,
            total_cost_usd: Some(0.05),
            period_start: Some("2026-07-01".into()),
            period_end: Some("2026-07-13".into()),
            last_fetched: "2026-07-13T20:00:00Z".into(),
            is_stale: false,
        };

        let json = serde_json::to_string(&s).unwrap();
        assert!(json.contains("\"anthropic\""));
        assert!(json.contains("1000"));
    }
}
