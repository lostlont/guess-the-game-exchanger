#[cfg(test)]
mod tests
{
	use
	{
		std::path::Path,
		configparser::ini::Ini,
		rusqlite,
		core::browser::
		{
			entry::Entry,
			firefox::Firefox,
		},
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
			|_path| create_db(None),
			false);

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
			|_path| create_db(None),
			false);

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
			|_path| create_db(None),
			false);

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
			|_path| create_db(None),
			false);

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
		let db_content = format!("
insert into data values ('item1', 2, 1, 0, 0, X'{ok_blob}');
insert into data values ('item2', 4, 1, 0, 0, X'{fail_blob}');
");

		let firefox = Firefox::try_new(
			&firefox_dir,
			|_path| Ok(create_ini(ini_content)),
			move |_path| create_db(Some(&db_content)),
			false)
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

	#[test]
	fn import_adds_entries_to_table_for_valid_entries()
	{
		let firefox_dir = Path::new("test/firefox");

		let ini_content = r#"
[Install]
Default = path/to/profile
"#;

		// The scope of the two connections should overlap.
		let db = create_shared_db(None);

		let firefox = Firefox::try_new(
			&firefox_dir,
			|_path| Ok(create_ini(ini_content)),
			move |_path| create_shared_db(None),
			false)
			.unwrap();

		let entries = vec![
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
		];

		firefox
			.import(entries.clone())
			.unwrap();
	
		let actual = db
			.unwrap()
			.prepare("select key, utf16_length, value from data")
			.unwrap()
			.query_map((), |row| Ok(Entry
				{
					key: row.get::<_, String>(0)?,
					utf16_length: row.get::<_, i64>(1)?,
					value: Vec::from(row.get::<_, Vec<u8>>(2)?),
				}))
			.unwrap()
			.collect::<Result<Vec<_>, _>>()
			.unwrap();

		assert_eq!(actual, entries);
	}

	fn create_ini(content: &str) -> Ini
	{
		let mut ini = Ini::new();
		ini.read(content.to_string())
			.unwrap();

		ini
	}

	fn create_db(content: Option<&str>) -> Result<rusqlite::Connection, rusqlite::Error>
	{
		let flags = rusqlite::OpenFlags::default();

		create_db_with(flags, content)
	}

	fn create_shared_db(content: Option<&str>) -> Result<rusqlite::Connection, rusqlite::Error>
	{
		let flags = rusqlite::OpenFlags::default() | rusqlite::OpenFlags::SQLITE_OPEN_SHARED_CACHE;

		create_db_with(flags, content)
	}

	fn create_db_with(flags: rusqlite::OpenFlags, content: Option<&str>) -> Result<rusqlite::Connection, rusqlite::Error>
	{
		// ::open_in_memory_with_flags() seems to be not enough for sharing memory. "file::memory:" path solves it.
		let db = rusqlite::Connection::open_with_flags("file::memory:", flags)?;

		let create_data_schema = "create table if not exists data (\
			key text primary key, \
			utf16_length integer not null, \
			conversion_type integer not null, \
			compression_type integer not null, \
			last_access_time integer not null default 0, \
			value blob not null)";
		db.execute(create_data_schema, ())?;

		if let Some(content) = content
		{
			db.execute_batch(content)?;
		}
		
		Ok(db)
	}
}
