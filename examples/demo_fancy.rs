/* examples/demo_fancy.rs */

use configme::*;

#[tokio::main]
async fn main() {
	init_config!(name = "fancyapp", where = "home", hide = true);
	create_subdirs!("data", "backup");
	create_file!("config.yaml");

	let dir = get_config_dir();
	println!("Config directory: {}", dir.display());
}
