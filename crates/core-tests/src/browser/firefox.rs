#[cfg(test)]
mod tests
{
    use core::browser::entry::Entry;

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
			|_path| create_db(""));

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
			|_path| create_db(""));

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
			|_path| create_db(""));

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
			|_path| create_db(""));

		assert!(firefox.is_ok());
	}

	#[test]
	fn export_returns_entries_for_valid_db()
	{
		let firefox_dir = Path::new("test/firefox");

		let ini_content = r#"
[Install]
Default = path/to/profile
"#;

		let ok_blob = hex::encode("ok");
		let fail_blob = hex::encode("fail");
		let content = format!("
create table data (key text, utf16_length integer, value blob);
insert into data values ('item1', 2, X'{ok_blob}');
insert into data values ('item2', 4, X'{fail_blob}');
");

		let firefox = Firefox::try_new(
			&firefox_dir,
			|_path| Ok(create_ini(ini_content)),
			move |_path| create_db(&content))
			.unwrap();

		let entries = firefox.export()
			.unwrap();

		assert_eq!(
			entries,
			vec![
				Entry
				{
					key:"item1".to_string(),
					utf16_length: 2,
					value: "ok".as_bytes().iter().cloned().collect(),
				},
				Entry
				{
					key:"item2".to_string(),
					utf16_length: 4,
					value: "fail".as_bytes().iter().cloned().collect(),
				},
			]);
	}

	fn create_ini(content: &str) -> Ini
	{
		let mut ini = Ini::new();
		ini.read(content.to_string())
			.unwrap();

		ini
	}

	fn create_db(content: &str) -> Result<sqlite::Connection, ()>
	{
		let db = sqlite::open(":memory:")
			.unwrap();

		db
			.execute(content)
			.unwrap();
		
		Ok(db)
	}
}
