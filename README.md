# configme

[![Crates.io](https://img.shields.io/crates/v/configme.svg)](https://crates.io/crates/configme)

**configme** is a Rust library that simplifies setting up your application's configuration directory and SQLite databases. It ensures directories and files are created if they don't exist, handling paths with tilde expansion for user home directories. It's designed for asynchronous applications and supports optional features for fancy logging and SQLite integration.

## Features

- Automatically create configuration directories in the user's home or `/opt`.
- Option to hide the directory with a leading dot (e.g., `~/.appname`).
- Create subdirectories and empty files within the config directory.
- Optional SQLite support to create database files asynchronously.
- Optional fancy logging for colorful output.
- Built on Tokio for async operations.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
configme = "1"
```

For optional features:

- Fancy logging: `configme = { version = "1", features = ["fancy"] }`
- SQLite support: `configme = { version = "1", features = ["sqlite"] }`
- Both: `configme = { version = "1", features = ["fancy", "sqlite"] }`

## Usage

The library provides macros for easy setup. All operations are designed to be idempotent—running them multiple times won't recreate existing files or directories.

### Basic Setup

Use the `init_config!` macro to initialize the config directory. It sets a global path that other macros use.

```rust
use configme::*;

#[tokio::main]
async fn main() {
    // Initializes ~/.configme (using crate name by default)
    init_config!();

    // Create subdirectories
    create_subdirs!("cache", "logs");

    // Create an empty file
    create_file!("settings.json");

    // Get the config directory path
    let dir = get_config_dir();
    println!("Config directory: {}", dir.display());
}
```

### Customizing Initialization

You can customize the directory name, location, and visibility:

- `name`: Custom app name (defaults to crate name via `env!("CARGO_PKG_NAME")`).
- `where`: "home" (default, e.g., `~/appname`) or "opt" (e.g., `/opt/appname`).
- `hide`: `true` to add a leading dot (e.g., `~/.appname`).

```rust
use configme::*;

#[tokio::main]
async fn main() {
    // Initializes ~/.example in home directory, hidden
    init_config!(name = "example", where = "home", hide = true);

    let dir = get_config_dir();
    println!("Config directory: {}", dir.display());
}
```

### SQLite Support (Requires "sqlite" Feature)

Create an empty SQLite database file asynchronously. It ensures the parent directory exists.

```rust
use configme::*;

#[tokio::main]
async fn main() {
    init_config!();
    create_subdirs!("db");

    // Creates db/app.sqlite if it doesn't exist
    sqlite!("db/app.sqlite").await;

    let dir = get_config_dir();
    println!("Config directory: {}", dir.display());
}
```

### Fancy Logging (Requires "fancy" Feature)

When enabled, logs use colorful output via the `fancy-log` crate. Without it, logs fall back to plain `println!` or `eprintln!`.

## Examples

The crate includes example files in the `examples/` directory:

- `demo.rs`: Basic directory and file creation.
- `demo_sqlite.rs`: SQLite database creation (requires "sqlite" feature).
- `demo_fancy.rs`: Custom hidden directory with fancy logging (requires "fancy" feature).

Run them with Cargo:

```sh
cargo run --example demo
cargo run --example demo_sqlite --features sqlite
cargo run --example demo_fancy --features fancy
```

## Dependencies

- Required: `tokio` (for async runtime), `shellexpand` (for path expansion).
- Optional: `fancy-log` (for "fancy" feature), `sqlx` (for "sqlite" feature).

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please open an issue or pull request on [GitHub](https://github.com/canmi21/configme).

## Acknowledgments

- Built with Rust's macro system for declarative setup.
- Thanks to the Rust community for crates like `tokio`, `sqlx`, and `shellexpand`.