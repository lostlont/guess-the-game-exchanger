#[cfg(test)]
mod tests
{
	use
	{
		std::path::Path,
		core::browser::chrome::Chrome,
	};

	#[test]
	fn try_new_returns_browser()
	{
		let chrome_dir = Path::new("test/chrome");

		let chrome = Chrome::try_new(
			&chrome_dir,
			|_path| create_db());

		assert!(chrome.is_ok());
	}

	fn create_db() -> Result<rusty_leveldb::DB, rusty_leveldb::Status>
	{
		let options = rusty_leveldb::in_memory();
		rusty_leveldb::DB::open(".", options)
	}
}
