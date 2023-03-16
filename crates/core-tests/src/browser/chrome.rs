#[cfg(test)]
mod tests
{
	use
	{
		std::path::Path,
		core::
		{
			browser::chrome::Chrome,
			profile::ProfileEntry,
		},
	};

	#[test]
	fn try_new_returns_browser()
	{
		let chrome_dir = Path::new("test/chrome");

		let chrome = Chrome::try_new(
			&chrome_dir,
			|_path| create_db(None));

		assert!(chrome.is_ok());
	}

	#[test]
	fn export_returns_entries()
	{
		let chrome_dir = Path::new("test/chrome");

		let db_content = vec![
			("item1".as_bytes(), "ok".as_bytes()),
			("item2".as_bytes(), "fail".as_bytes()),
		];
		let chrome = Chrome::try_new(
			&chrome_dir,
			move |_path| create_db(Some(db_content.as_slice())))
			.unwrap();

		let profile = chrome
			.export()
			.unwrap();

		let actual = profile
			.get_entries()
			.cloned()
			.collect::<Vec<_>>();

		let expected = vec![
			ProfileEntry
			{
				key:"item1".to_string(),
				value: "ok".to_string(),
			},
			ProfileEntry
			{
				key:"item2".to_string(),
				value: "fail".to_string(),
			},
		];
		assert_eq!(actual, expected);
	}

	fn create_db<'a>(content: Option<&[(&'a [u8], &'a [u8])]>) -> Result<rusty_leveldb::DB, rusty_leveldb::Status>
	{
		let options = rusty_leveldb::in_memory();
		let mut db = rusty_leveldb::DB::open("testdb", options)
			.unwrap();

		if let Some(content) = content
		{
			for (key, value) in content
			{
				db
					.put(key, value)
					.unwrap();
			}
		}

		Ok(db)
	}
}
