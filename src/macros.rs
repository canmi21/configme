/* src/macros.rs */

// Internal helper macro to parse key-value pairs without type conflicts.
// This is not exported and is only visible within this crate.
#[doc(hidden)]
#[macro_export]
macro_rules! __configme_parse_kv {
	// Match 'name = <value>' and assign to the `name` field.
	($name:ident, $location:ident, $hide:ident, name = $value:expr) => {
		$name = Some($value.to_string());
	};
	// Match 'where = <value>' and assign to the `location` field.
	($name:ident, $location:ident, $hide:ident, where = $value:expr) => {
		$location = Some($value);
	};
	// Match 'hide = <value>' and assign to the `hide` field.
	($name:ident, $location:ident, $hide:ident, hide = $value:expr) => {
		$hide = Some($value);
	};
}

/// Initializes the application's configuration directory.
#[macro_export]
macro_rules! init_config {
    // This single arm handles both zero and multiple arguments.
    ($($key:ident = $value:expr),* $(,)?) => {{
        let mut name: Option<String> = None;
        let mut location: Option<&str> = None;
        let mut hide: Option<bool> = None;

        // For each key-value pair, invoke the helper macro.
        // This isolates the type checking for each assignment.
        $(
            $crate::__configme_parse_kv!(name, location, hide, $key = $value);
        )*

        let final_name = name.unwrap_or_else(|| env!("CARGO_PKG_NAME").to_string());
        let final_location = location.unwrap_or("home");
        let final_hide = hide.unwrap_or(false);

        let dir_name = if final_hide { format!(".{}", final_name) } else { final_name };

        let base_path_str = match final_location {
            "opt" => format!("/opt/{}", dir_name),
            _ => format!("~/{}", dir_name),
        };

        let config_path = $crate::utils::expand_path(base_path_str);

        $crate::utils::ensure_dir(&config_path);
        $crate::utils::set_config_dir(config_path);
    }};
}

/// Creates one or more subdirectories inside the main configuration directory.
#[macro_export]
macro_rules! create_subdirs {
    ($($dir:expr),+ $(,)?) => {{
        let base = $crate::get_config_dir();
        $(
            let path = base.join($dir);
            $crate::utils::ensure_dir(&path);
        )+
    }};
}

/// Creates an empty file inside the main configuration directory.
#[macro_export]
macro_rules! create_file {
	($name:expr) => {{
		let base = $crate::get_config_dir();
		let path = base.join($name);
		$crate::utils::ensure_file(&path);
	}};
}

/// Creates an empty SQLite database file inside the configuration directory.
#[cfg(feature = "sqlite")]
#[macro_export]
macro_rules! sqlite {
	($filename:expr) => {
		async {
			let base = $crate::get_config_dir();
			let db_path = base.join($filename);

			if let Some(parent) = db_path.parent() {
				$crate::utils::ensure_dir(parent);
			}

			$crate::sqlite::ensure_sqlite(&db_path).await;
		}
	};
}
