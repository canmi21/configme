/* src/lib.rs */

// These modules contain functions called by the macros, so they must be public.
#[cfg(feature = "sqlite")]
pub mod sqlite;
pub mod utils;

// This module contains the macros themselves.
// `#[macro_export]` handles making them available at the crate root.
mod macros;

// Re-export the main function for user convenience.
pub use utils::get_config_dir;
