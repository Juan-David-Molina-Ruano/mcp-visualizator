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

/// Current time as ISO 8601 UTC string (e.g., `2026-07-15T01:09:22Z`).
fn chrono_now() -> String {
    utc_now_iso()
}

/// Convert epoch seconds to ISO 8601 UTC string using Hinnant's algorithm.
fn utc_now_iso() -> String {
    let secs = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();

    let days = (secs / 86400) as i64;
    let time_of_day = secs % 86400;
    let h = time_of_day / 3600;
    let m = (time_of_day % 3600) / 60;
    let s = time_of_day % 60;

    // Howard Hinnant's date algorithm (public domain)
    let z = days + 719468;
    let era = (if z >= 0 { z } else { z - 146096 }) / 146097;
    let doe = (z - era * 146097) as u64;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let y = yoe as i64 + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let mo = if mp < 10 { mp + 3 } else { mp - 9 };
    let y = if mo <= 2 { y + 1 } else { y };

    format!("{y:04}-{mo:02}-{d:02}T{h:02}:{m:02}:{s:02}Z")
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
    fn chrono_now_returns_valid_iso8601() {
        let ts = chrono_now();
        // Must match YYYY-MM-DDTHH:MM:SSZ
        assert_eq!(ts.len(), 20, "ISO 8601 UTC should be 20 chars: {ts}");
        assert!(ts.ends_with('Z'), "must end with Z: {ts}");
        assert_eq!(&ts[4..5], "-", "dash after year");
        assert_eq!(&ts[7..8], "-", "dash after month");
        assert_eq!(&ts[10..11], "T", "T separator");
        assert_eq!(&ts[13..14], ":", "colon after hours");
        assert_eq!(&ts[16..17], ":", "colon after minutes");
        // Year must be reasonable (2024–2099)
        let year: i32 = ts[..4].parse().expect("year parseable");
        assert!(year >= 2024 && year <= 2099, "year out of range: {year}");
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
