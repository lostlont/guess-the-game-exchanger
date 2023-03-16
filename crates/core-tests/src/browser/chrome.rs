#[cfg(test)]
mod tests
{
	use
	{
		std::path::Path,
		core::
		{
			browser::chrome::
			{
				self,
				Chrome,
			},
			profile::ProfileEntry,
		},
		crate::browser::tests::
		{
			VALID_ENTRIES,
			INVALID_ENTRIES,
		},
	};

	#[test]
	fn try_new_returns_browser()
	{
		let chrome_dir = Path::new("test/chrome");

		let chrome = Chrome::try_new(
			&chrome_dir,
			|_path| create_db(None),
			false);

		assert!(chrome.is_ok());
	}

	#[test]
	fn export_returns_entries_filtered()
	{
		let chrome_dir = Path::new("test/chrome");

		let page_entries : Vec<(Vec<u8>, Vec<u8>)> = std::iter::Iterator
			::chain(
				VALID_ENTRIES.iter(),
				INVALID_ENTRIES.iter())
			.map(|(key, value)|
			{
				let key = chrome::KEY_PREFIX.to_string() + key;
				let value = chrome::VALUE_PREFIX.to_string() + value;
				(key.bytes().collect(), value.bytes().collect())
			})
			.collect::<Vec<_>>();
		let other_entries : Vec<(Vec<u8>, Vec<u8>)> = vec![
			("META:chrome://settings".bytes().collect(), vec![192, 255]),
			("META:https://guessthe.game".bytes().collect(), "?".bytes().collect()),
			("VERSION".bytes().collect(), "1".bytes().collect()),
			("_chrome://settings\0\u{1}signin-promo-count".bytes().collect(), "\u{1}1".bytes().collect()),
			("_https://other.page\0\u{1}301_gamestate".bytes().collect(), "\u{1}x".bytes().collect()),
		];
		let mut db_content = std::iter::Iterator
			::chain(
				page_entries.iter(),
				other_entries.iter())
			.cloned()
			.collect::<Vec<_>>();
		db_content.sort();

		let chrome = Chrome::try_new(
			&chrome_dir,
			move |_path|
			{
				let db_content = db_content
					.iter()
					.map(|(key, value)| (&key[..], &value[..]))
					.collect::<Vec<_>>();
				create_db(Some(db_content.as_slice()))
			},
			false)
			.unwrap();

		let profile = chrome
			.export()
			.unwrap();

		let mut actual = profile
			.get_entries()
			.cloned()
			.collect::<Vec<_>>();
		actual.sort_by(|a, b| a.key.cmp(&b.key));

		let mut expected = VALID_ENTRIES
			.iter()
			.map(|(key, value)| ProfileEntry
			{
				key: key.to_string(),
				value: value.to_string(),
			})
			.collect::<Vec<_>>();
		expected.sort_by(|a, b| a.key.cmp(&b.key));

		assert_eq!(actual, expected);
	}

	// TODO Test Chrome::import()
	// It would require either more instances of the database with shared memory, like with SQLite - which is not doable with LevelDB,
	// or changing the interface of the functions to shared references - which is fishy,
	// or using traits to fully mock objects.

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
