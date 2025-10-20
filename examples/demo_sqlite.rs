/* examples/demo_sqlite.rs */

use configme::*;

#[tokio::main]
async fn main() {
	init_config!(name = "example", where = "home");
	create_subdirs!("db");
	sqlite!("db/app.sqlite").await;

	let dir = get_config_dir();
	println!("Config directory: {}", dir.display());
}
