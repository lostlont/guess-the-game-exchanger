use
{
	std::path::PathBuf,
	rusty_leveldb::
	{
		self,
		LdbIterator,
	},
	crate::browser::
	{
		Browser,
		Entry,
	},
};

pub struct Chrome;

impl Chrome
{
	pub fn try_new() -> Result<Option<Box<dyn Browser>>, String>
	{
		// TODO Move get_chrome_dir out of try_new
		let chrome_dir = get_chrome_dir()?;

		let result = if let Some(chrome_dir) = chrome_dir
		{
			let db_dir = chrome_dir.join("User Data/Default/Local Storage/leveldb");
			let options = rusty_leveldb::Options
			{
				create_if_missing: false,
				..rusty_leveldb::Options::default()
			};
			let mut db = rusty_leveldb::DB::open(db_dir, options)
				.or(Err("1".to_string()))?; // TODO
			let mut iter = db.new_iter().unwrap();
			while let Some((key, value)) = iter.next()
			{
				let key = String::from_utf8(key).unwrap_or("?".to_string());
				let value = String::from_utf8(value).unwrap_or("?".to_string());
				println!("{key:?}: {value:?}");
			}

			let chrome: Box<dyn Browser> = Box::new(Chrome
			{
			});

			Some(chrome)
		}
		else
		{
			None
		};

		Ok(result)
	}
}

impl Browser for Chrome
{
	fn name(&self) -> &str
	{
		"Chrome"
	}

	fn export(&self) -> Result<Vec<Entry>, String>
	{
		todo!()
	}

	fn import(&self, entries: Vec<Entry>) -> Result<(), String>
	{
		todo!()
	}
}

fn get_chrome_dir() -> Result<Option<PathBuf>, String>
{
	let data_dir = dirs::data_local_dir();
	let data_dir = data_dir
		.ok_or("No data directory is present!".to_string())?;
	let chrome_dir = data_dir.join("Google/Chrome");

	let result = if chrome_dir.exists()
	{
		Some(chrome_dir)
	}
	else
	{
		None
	};
	Ok(result)
}
