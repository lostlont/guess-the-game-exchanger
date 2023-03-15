use
{
	std::path::
	{
		Path,
		PathBuf,
	},
	rusty_leveldb::
	{
		self,
		LdbIterator,
	},
	crate::
	{
		browser::Browser,
		profile::Profile,
	},
};

pub struct Chrome
{
	db_dir: PathBuf,
}

impl Chrome
{
	pub fn try_new(chrome_dir: &Path) -> Result<Box<dyn Browser>, String>
	{
		let db_dir = chrome_dir.join("User Data/Default/Local Storage/leveldb");
		/*
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
		*/

		let chrome: Box<dyn Browser> = Box::new(Chrome
		{
			db_dir,
		});

		Ok(chrome)
	}
}

impl Browser for Chrome
{
	fn name(&self) -> &str
	{
		"Chrome"
	}

	fn export(&self) -> Result<Profile, String>
	{
		todo!()
	}

	fn import(&self, profile: Profile) -> Result<(), String>
	{
		todo!()
	}
}

pub fn get_chrome_dir() -> Result<Option<PathBuf>, String>
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
