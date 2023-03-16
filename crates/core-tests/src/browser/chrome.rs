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
	fn export_returns_entries_filtered()
	{
		let chrome_dir = Path::new("test/chrome");

		let db_content = vec![
			("META:chrome://settings".as_bytes(), &[192, 255][..]),
			("META:https://guessthe.game".as_bytes(), "?".as_bytes()),
			("VERSION".as_bytes(), "1".as_bytes()),
			("_chrome://settings\0\u{1}signin-promo-count".as_bytes(), "\u{1}1".as_bytes()),
			("_https://guessthe.game\0\u{1}255_gamestate".as_bytes(), "\u{1}win".as_bytes()),
			("_https://guessthe.game\0\u{1}255_guess1".as_bytes(), "\u{1}Game Name".as_bytes()),
			("_https://guessthe.game\0\u{1}300_gamestate".as_bytes(), "\u{1}lose".as_bytes()),
			("_https://guessthe.game\0\u{1}300_guess1".as_bytes(), "\u{1}Skipped!".as_bytes()),
			("_https://guessthe.game\0\u{1}300_guess2".as_bytes(), "\u{1}Skipped!".as_bytes()),
			("_https://guessthe.game\0\u{1}CMPList".as_bytes(), "\u{1}{\"key\":\"value\"}".as_bytes()),
			("_https://guessthe.game\0\u{1}_cmpRepromptHash".as_bytes(), "\u{1}ABC".as_bytes()),
			("_https://guessthe.game\0\u{1}currentstreak".as_bytes(), "\u{1}0".as_bytes()),
			("_https://guessthe.game\0\u{1}noniabvendorconsent".as_bytes(), "\u{1}consent".as_bytes()),
			("_https://guessthe.game\0\u{1}totalplayed".as_bytes(), "\u{1}2".as_bytes()),
			("_https://other.page\0\u{1}301_gamestate".as_bytes(), "\u{1}x".as_bytes()),
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
				key:"255_gamestate".to_string(),
				value: "win".to_string(),
			},
			ProfileEntry
			{
				key:"255_guess1".to_string(),
				value: "Game Name".to_string(),
			},
			ProfileEntry
			{
				key:"300_gamestate".to_string(),
				value: "lose".to_string(),
			},
			ProfileEntry
			{
				key:"300_guess1".to_string(),
				value: "Skipped!".to_string(),
			},
			ProfileEntry
			{
				key:"300_guess2".to_string(),
				value: "Skipped!".to_string(),
			},
			ProfileEntry
			{
				key:"currentstreak".to_string(),
				value: "0".to_string(),
			},
			ProfileEntry
			{
				key:"totalplayed".to_string(),
				value: "2".to_string(),
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
