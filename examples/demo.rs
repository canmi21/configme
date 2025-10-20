/* examples/demo.rs */

use configme::*;

#[tokio::main]
async fn main() {
	init_config!();
	create_subdirs!("cache", "logs");
	create_file!("settings.json");

	let dir = get_config_dir();
	println!("Config directory: {}", dir.display());
}
