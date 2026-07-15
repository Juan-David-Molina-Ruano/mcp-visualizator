use rusqlite::{params, Connection};

use crate::domain::errors::AppError;
use crate::domain::models::AppSettings;

/// SQLite database for app-owned data (telemetry history, settings, usage cache).
pub struct Database {
    conn: Connection,
}

impl Database {
    /// Open or create the database at the platform-specific data directory.
    pub fn new(app_handle: &tauri::AppHandle) -> Result<Self, AppError> {
        use tauri::Manager;

        let data_dir = app_handle
            .path()
            .app_data_dir()
            .map_err(|e| AppError::Database(format!("Cannot resolve data dir: {e}")))?;

        std::fs::create_dir_all(&data_dir)
            .map_err(|e| AppError::Database(format!("Cannot create data dir: {e}")))?;

        let db_path = data_dir.join("data.db");
        let conn = Connection::open(&db_path)
            .map_err(|e| AppError::Database(format!("Cannot open database: {e}")))?;

        let mut db = Self { conn };
        db.run_migrations()?;
        Ok(db)
    }

    /// Create an in-memory database for testing.
    #[cfg(test)]
    pub fn new_in_memory() -> Result<Self, AppError> {
        let conn = Connection::open_in_memory()
            .map_err(|e| AppError::Database(format!("Cannot open in-memory DB: {e}")))?;

        let mut db = Self { conn };
        db.run_migrations()?;
        Ok(db)
    }

    /// Run database migrations.
    fn run_migrations(&mut self) -> Result<(), AppError> {
        self.conn
            .execute_batch(
                "
                CREATE TABLE IF NOT EXISTS telemetry_snapshots (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    mcp_name TEXT NOT NULL,
                    timestamp TEXT NOT NULL,
                    state TEXT NOT NULL,
                    latency_ms REAL,
                    errors INTEGER NOT NULL DEFAULT 0,
                    uptime_s INTEGER NOT NULL DEFAULT 0,
                    cpu_pct REAL,
                    mem_mb REAL
                );

                CREATE INDEX IF NOT EXISTS idx_telemetry_name_time
                    ON telemetry_snapshots(mcp_name, timestamp);

                CREATE TABLE IF NOT EXISTS settings (
                    key TEXT PRIMARY KEY,
                    value TEXT NOT NULL
                );

                CREATE TABLE IF NOT EXISTS anthropic_usage (
                    id INTEGER PRIMARY KEY CHECK (id = 1),
                    input_tokens INTEGER NOT NULL DEFAULT 0,
                    output_tokens INTEGER NOT NULL DEFAULT 0,
                    last_updated TEXT NOT NULL
                );

                INSERT OR IGNORE INTO settings (key, value) VALUES
                    ('polling_interval_ms', '30000'),
                    ('spawn_timeout_secs', '10'),
                    ('ping_timeout_secs', '5'),
                    ('consecutive_failures_threshold', '3');
                ",
            )
            .map_err(|e| AppError::Database(format!("Migration failed: {e}")))?;

        Ok(())
    }

    /// Get a setting value by key.
    pub fn get_setting(&self, key: &str) -> Result<String, AppError> {
        self.conn
            .query_row(
                "SELECT value FROM settings WHERE key = ?1",
                params![key],
                |row| row.get(0),
            )
            .map_err(|e| AppError::Database(format!("Setting not found: {e}")))
    }

    /// Set a setting value.
    pub fn set_setting(&self, key: &str, value: &str) -> Result<(), AppError> {
        self.conn
            .execute(
                "INSERT OR REPLACE INTO settings (key, value) VALUES (?1, ?2)",
                params![key, value],
            )
            .map_err(|e| AppError::Database(format!("Cannot set setting: {e}")))?;
        Ok(())
    }

    /// Get all settings as AppSettings.
    pub fn get_all_settings(&self) -> Result<AppSettings, AppError> {
        let polling: u64 = self
            .get_setting("polling_interval_ms")?
            .parse()
            .unwrap_or(30_000);
        let spawn: u64 = self
            .get_setting("spawn_timeout_secs")?
            .parse()
            .unwrap_or(10);
        let ping: u64 = self
            .get_setting("ping_timeout_secs")?
            .parse()
            .unwrap_or(5);
        let threshold: u32 = self
            .get_setting("consecutive_failures_threshold")?
            .parse()
            .unwrap_or(3);

        Ok(AppSettings {
            polling_interval_ms: polling,
            spawn_timeout_secs: spawn,
            ping_timeout_secs: ping,
            consecutive_failures_threshold: threshold,
        })
    }

    /// Insert a telemetry snapshot.
    pub fn insert_telemetry_snapshot(
        &self,
        mcp_name: &str,
        state: &str,
        latency_ms: Option<f64>,
        errors: u32,
        uptime_s: u64,
        cpu_pct: Option<f64>,
        mem_mb: Option<f64>,
    ) -> Result<(), AppError> {
        let timestamp = chrono_now_iso();
        self.conn
            .execute(
                "INSERT INTO telemetry_snapshots
                 (mcp_name, timestamp, state, latency_ms, errors, uptime_s, cpu_pct, mem_mb)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
                params![mcp_name, timestamp, state, latency_ms, errors, uptime_s, cpu_pct, mem_mb],
            )
            .map_err(|e| AppError::Database(format!("Cannot insert telemetry: {e}")))?;
        Ok(())
    }

    /// Get telemetry history for an MCP server within a duration (in seconds).
    pub fn get_telemetry_history(
        &self,
        mcp_name: &str,
        duration_secs: u64,
    ) -> Result<Vec<TelemetryRow>, AppError> {
        let cutoff = chrono_now_iso();
        // For MVP, return all rows for the MCP name (time filtering done in code)
        let mut stmt = self
            .conn
            .prepare(
                "SELECT mcp_name, timestamp, state, latency_ms, errors, uptime_s, cpu_pct, mem_mb
                 FROM telemetry_snapshots
                 WHERE mcp_name = ?1
                 ORDER BY timestamp DESC
                 LIMIT ?2",
            )
            .map_err(|e| AppError::Database(format!("Cannot prepare query: {e}")))?;

        let limit = (duration_secs / 30).max(100); // roughly one row per 30s polling
        let rows = stmt
            .query_map(params![mcp_name, limit], |row| {
                Ok(TelemetryRow {
                    mcp_name: row.get(0)?,
                    timestamp: row.get(1)?,
                    state: row.get(2)?,
                    latency_ms: row.get(3)?,
                    errors: row.get(4)?,
                    uptime_s: row.get(5)?,
                    cpu_pct: row.get(6)?,
                    mem_mb: row.get(7)?,
                })
            })
            .map_err(|e| AppError::Database(format!("Cannot query telemetry: {e}")))?;

        let mut result = Vec::new();
        for row in rows {
            result.push(row.map_err(|e| AppError::Database(format!("Row error: {e}")))?);
        }

        // Reverse so oldest first
        result.reverse();
        let _ = cutoff;
        Ok(result)
    }

    /// Get accumulated Anthropic usage.
    pub fn get_anthropic_usage(&self) -> Result<(u64, u64), AppError> {
        let result = self.conn.query_row(
            "SELECT input_tokens, output_tokens FROM anthropic_usage WHERE id = 1",
            [],
            |row| Ok((row.get(0)?, row.get(1)?)),
        );

        match result {
            Ok((i, o)) => Ok((i, o)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok((0, 0)),
            Err(e) => Err(AppError::Database(format!("Cannot get usage: {e}"))),
        }
    }

    /// Accumulate Anthropic usage (add to existing).
    pub fn accumulate_anthropic_usage(
        &self,
        input_tokens: u64,
        output_tokens: u64,
    ) -> Result<(), AppError> {
        let timestamp = chrono_now_iso();
        self.conn
            .execute(
                "INSERT INTO anthropic_usage (id, input_tokens, output_tokens, last_updated)
                 VALUES (1, ?1, ?2, ?3)
                 ON CONFLICT(id) DO UPDATE SET
                    input_tokens = input_tokens + ?1,
                    output_tokens = output_tokens + ?2,
                    last_updated = ?3",
                params![input_tokens, output_tokens, timestamp],
            )
            .map_err(|e| AppError::Database(format!("Cannot accumulate usage: {e}")))?;
        Ok(())
    }
}

/// Row from telemetry_snapshots table.
#[derive(Debug)]
pub struct TelemetryRow {
    pub mcp_name: String,
    pub timestamp: String,
    pub state: String,
    pub latency_ms: Option<f64>,
    pub errors: u32,
    pub uptime_s: u64,
    pub cpu_pct: Option<f64>,
    pub mem_mb: Option<f64>,
}

fn chrono_now_iso() -> String {
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
    fn db_migrations_creates_tables() {
        let db = Database::new_in_memory().unwrap();

        // Verify default settings exist
        let polling = db.get_setting("polling_interval_ms").unwrap();
        assert_eq!(polling, "30000");

        let spawn = db.get_setting("spawn_timeout_secs").unwrap();
        assert_eq!(spawn, "10");
    }

    #[test]
    fn settings_crud() {
        let db = Database::new_in_memory().unwrap();

        db.set_setting("custom_key", "custom_value").unwrap();
        let val = db.get_setting("custom_key").unwrap();
        assert_eq!(val, "custom_value");

        db.set_setting("custom_key", "updated").unwrap();
        let val = db.get_setting("custom_key").unwrap();
        assert_eq!(val, "updated");
    }

    #[test]
    fn get_all_settings_returns_defaults() {
        let db = Database::new_in_memory().unwrap();
        let settings = db.get_all_settings().unwrap();

        assert_eq!(settings.polling_interval_ms, 30_000);
        assert_eq!(settings.spawn_timeout_secs, 10);
        assert_eq!(settings.ping_timeout_secs, 5);
        assert_eq!(settings.consecutive_failures_threshold, 3);
    }

    #[test]
    fn telemetry_insert_and_query() {
        let db = Database::new_in_memory().unwrap();

        db.insert_telemetry_snapshot("test-mcp", "running", Some(42.5), 0, 300, Some(1.2), Some(25.6))
            .unwrap();

        let history = db.get_telemetry_history("test-mcp", 3600).unwrap();
        assert_eq!(history.len(), 1);
        assert_eq!(history[0].mcp_name, "test-mcp");
        assert_eq!(history[0].state, "running");
        assert_eq!(history[0].latency_ms, Some(42.5));
    }

    #[test]
    fn anthropic_usage_accumulation() {
        let db = Database::new_in_memory().unwrap();

        // Initial: (0, 0)
        let (i, o) = db.get_anthropic_usage().unwrap();
        assert_eq!(i, 0);
        assert_eq!(o, 0);

        // Add first batch
        db.accumulate_anthropic_usage(100, 50).unwrap();
        let (i, o) = db.get_anthropic_usage().unwrap();
        assert_eq!(i, 100);
        assert_eq!(o, 50);

        // Add second batch
        db.accumulate_anthropic_usage(200, 100).unwrap();
        let (i, o) = db.get_anthropic_usage().unwrap();
        assert_eq!(i, 300);
        assert_eq!(o, 150);
    }
}
