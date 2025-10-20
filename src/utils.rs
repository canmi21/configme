/* src/utils.rs */

use std::fs;
use std::path::{Path, PathBuf};
use std::sync::{OnceLock, RwLock};

#[cfg(feature = "fancy-log")]
use fancy_log::{LogLevel, log};

// Global static variable to hold the config directory path.
// OnceLock ensures it's initialized only once.
// RwLock allows for safe concurrent reads and exclusive writes.
static CONFIG_DIR: OnceLock<RwLock<Option<PathBuf>>> = OnceLock::new();

fn get_store() -> &'static RwLock<Option<PathBuf>> {
	CONFIG_DIR.get_or_init(|| RwLock::new(None))
}

/// Expands tilde (~) in a path to the user's home directory.
pub fn expand_path<P: AsRef<Path>>(path: P) -> PathBuf {
	let path_str = path.as_ref().to_string_lossy();
	let expanded = shellexpand::tilde(&path_str);
	PathBuf::from(expanded.to_string())
}

/// Sets the global configuration directory path. Should only be called by `init_config!`.
pub fn set_config_dir(path: PathBuf) {
	let store = get_store();
	let mut guard = store.write().unwrap();
	*guard = Some(path);
}

/// Gets the global configuration directory path.
///
/// # Panics
///
/// Panics if `init_config!` has not been called yet.
pub fn get_config_dir() -> PathBuf {
	let store = get_store();
	let guard = store.read().unwrap();
	guard
		.clone()
		.expect("Config directory has not been initialized. Call init_config! first.")
}

/// Ensures a directory exists, creating it if necessary.
pub fn ensure_dir(path: &Path) {
	if path.exists() {
		return;
	}
	match fs::create_dir_all(path) {
		Ok(_) => log_info(&format!("Created directory: {}", path.display())),
		Err(e) => log_error(&format!(
			"Failed to create directory {}: {}",
			path.display(),
			e
		)),
	}
}

/// Ensures a file exists, creating it if necessary.
pub fn ensure_file(path: &Path) {
	if path.exists() {
		return;
	}
	// Ensure parent directory exists before creating the file.
	if let Some(parent) = path.parent() {
		ensure_dir(parent);
	}
	match fs::File::create(path) {
		Ok(_) => log_info(&format!("Created file: {}", path.display())),
		Err(e) => log_error(&format!("Failed to create file {}: {}", path.display(), e)),
	}
}

/// Logs an informational message.
pub fn log_info(msg: &str) {
	#[cfg(feature = "fancy-log")]
	log(LogLevel::Info, msg);

	#[cfg(not(feature = "fancy-log"))]
	println!("{}", msg);
}

/// Logs an error message.
pub fn log_error(msg: &str) {
	#[cfg(feature = "fancy-log")]
	log(LogLevel::Error, msg);

	#[cfg(not(feature = "fancy-log"))]
	eprintln!("{}", msg);
}
