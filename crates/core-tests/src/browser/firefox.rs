#[cfg(test)]
mod tests
{
	use
	{
		std::path::Path,
		configparser::ini::Ini,
		sqlite,
		core::browser::firefox::Firefox,
	};

	#[test]
	fn try_new_returns_error_for_ini_without_install_section()
	{
		let firefox_dir = Path::new("test/firefox");

		let ini_content = r#"
	[Instal]
	Default = path/to/profile
	"#;

		let firefox = Firefox::try_new(
			&firefox_dir,
			|_path| Ok(create_ini(ini_content)),
			|_path| create_db());

		assert!(firefox.is_err());
	}

	#[test]
	fn try_new_returns_error_for_ini_without_default_key()
	{
		let firefox_dir = Path::new("test/firefox");

		let ini_content = r#"
	[Install]
	Defaul = path/to/profile
	"#;

		let firefox = Firefox::try_new(
			&firefox_dir,
			|_path| Ok(create_ini(ini_content)),
			|_path| create_db());

		assert!(firefox.is_err());
	}

	#[test]
	fn try_new_returns_error_for_ini_without_default_value()
	{
		let firefox_dir = Path::new("test/firefox");
		
		let ini_content = r#"
		[Install]
		Default
		"#;
		
		let firefox = Firefox::try_new(
			&firefox_dir,
			|_path| Ok(create_ini(ini_content)),
			|_path| create_db());

		assert!(firefox.is_err());
	}

	#[test]
	fn try_new_returns_browser_for_valid_ini()
	{
		let firefox_dir = Path::new("test/firefox");

		let ini_content = r#"
	[Install]
	Default = path/to/profile
	"#;

		let firefox = Firefox::try_new(
			&firefox_dir,
			|_path| Ok(create_ini(ini_content)),
			|_path| create_db());

		assert!(firefox.is_ok());
	}

	fn create_ini(content: &str) -> Ini
	{
		let mut ini = Ini::new();
		ini.read(content.to_string())
			.unwrap();

		ini
	}

	fn create_db() -> sqlite::Result<sqlite::Connection>
	{
		sqlite::open(":memory:")
	}
}
