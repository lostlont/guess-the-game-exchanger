#[cfg(test)]
mod tests
{
	use
	{
		std::path::Path,
		configparser::ini::Ini,
		rusqlite,
		core::
		{
			browser::firefox::Firefox,
			profile::
			{
				Profile,
				ProfileEntry,
			},
		},
		crate::browser::tests::
		{
			INVALID_ENTRIES,
			VALID_ENTRIES,
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

		let mut entries = std::iter::Iterator
			::chain(
				VALID_ENTRIES.iter(),
				INVALID_ENTRIES.iter())
			.collect::<Vec<_>>();
		entries.sort();

		let entries = entries
			.iter()
			.map(|(key, value)| insert_statement(key, value))
			.collect::<Vec<_>>();
		let db_content = entries.join("\n");

		let firefox = Firefox::try_new(
			&firefox_dir,
			|_path| Ok(create_ini(ini_content)),
			move |_path| create_db(Some(&db_content)),
			false)
			.unwrap();

		let profile = firefox
			.export()
			.unwrap();

		let actual = profile
			.get_entries()
			.cloned()
			.collect::<Vec<_>>();

		let mut expected = VALID_ENTRIES.clone();
		expected.sort();

		let expected = expected
			.iter()
			.map(|(key, value)| ProfileEntry
			{
				key: key.to_string(),
				value: value.to_string(),
			})
			.collect::<Vec<_>>();

		assert_eq!(
			actual,
			expected);
	}

	#[test]
	fn import_adds_entries_to_table_for_valid_entries()
	{
		let firefox_dir = Path::new("test/firefox");

		let ini_content = r#"
[Install]
Default = path/to/profile
"#;

		let db_content = insert_statement("existingkey", "existingvalue") + ";";

		// The scope of the two connections should overlap.
		let db = create_shared_db(Some(&db_content));

		let firefox = Firefox::try_new(
			&firefox_dir,
			|_path| Ok(create_ini(ini_content)),
			move |_path| create_shared_db(None),
			false)
			.unwrap();

		let entries = VALID_ENTRIES
			.iter()
			.map(|(key, value)| ProfileEntry
			{
				key: key.to_string(),
				value: value.to_string(),
			})
			.collect::<Vec<_>>();
		let profile = Profile::from(entries);

		firefox
			.import(profile)
			.unwrap();

		#[derive(Debug, PartialEq)]
		struct RawEntry
		{
			key: String,
			utf16_length: i64,
			value: Vec<u8>,
		}
	
		let actual = db
			.unwrap()
			.prepare("select key, utf16_length, value from data")
			.unwrap()
			.query_map((), |row| Ok(RawEntry
				{
					key: row.get::<_, String>(0)?,
					utf16_length: row.get::<_, i64>(1)?,
					value: row.get::<_, Vec<u8>>(2)?,
				}))
			.unwrap()
			.collect::<Result<Vec<_>, _>>()
			.unwrap();

		let expected = std::iter::Iterator
			::chain(
				vec![("existingkey", "existingvalue")].iter(),
				VALID_ENTRIES.iter())
			.map(|(key, value)| RawEntry
			{
				key: key.to_string(),
				utf16_length: value.len() as i64,
				value: value.bytes().collect(),
			})
			.collect::<Vec<_>>();
		assert_eq!(actual, expected);
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

	fn insert_statement(key: &str, value: &str) -> String
	{
		let length = value.len();
		let value_blob = hex::encode(value);
		format!("insert into data values ('{key}', '{length}', 1, 0, 0, X'{value_blob}');")
	}
}
