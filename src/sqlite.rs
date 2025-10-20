/* src/sqlite.rs */

use crate::utils::{log_error, log_info};
use sqlx::{Connection, SqliteConnection};
use std::path::Path;

/// Ensures a SQLite database file exists at the given path.
pub async fn ensure_sqlite(path: &Path) {
	if path.exists() {
		return;
	}

	// THE FIX: Add "?mode=rwc" to the connection string.
	// This explicitly tells the underlying SQLite C library to:
	// r: open for reading
	// w: open for writing
	// c: create the file if it does not exist.
	// This is more robust than relying on defaults, especially in protected directories.
	let uri = format!("sqlite://{}?mode=rwc", path.display());

	match SqliteConnection::connect(&uri).await {
		// The `unused_mut` warning is also fixed here by removing `mut conn`
		Ok(_) => {
			log_info(&format!("Created SQLite database: {}", path.display()));
		}
		Err(e) => log_error(&format!(
			"Failed to create SQLite database {}: {}",
			path.display(),
			e
		)),
	}
}
