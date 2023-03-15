use
{
	std::path::
	{
		Path,
		PathBuf,
	},
	crate::
	{
		browser::Browser,
		profile::Profile,
	},
};

pub struct Chrome<TDbProvider, TError>
where
	TDbProvider: Fn(&Path) -> Result<rusty_leveldb::DB, TError>,
{
	db_dir: PathBuf,
	db_provider: TDbProvider,
}

impl<TDbProvider, TError> Chrome<TDbProvider, TError>
where
	TDbProvider: Fn(&Path) -> Result<rusty_leveldb::DB, TError> + 'static,
	TError: 'static,
{
	pub fn try_new(chrome_dir: &Path, db_provider: TDbProvider) -> Result<Box<dyn Browser>, String>
	{
		let db_dir = chrome_dir.join("User Data/Default/Local Storage/leveldb");

		let chrome: Box<dyn Browser> = Box::new(Chrome
		{
			db_dir,
			db_provider,
		});

		Ok(chrome)
	}
}

impl<TDbProvider, TError> Browser for Chrome<TDbProvider, TError>
where
	TDbProvider: Fn(&Path) -> Result<rusty_leveldb::DB, TError>,
{
	fn name(&self) -> &str
	{
		"Chrome"
	}

	fn export(&self) -> Result<Profile, String>
	{
		let mut db = (self.db_provider)(&self.db_dir)
			.or(Err("Could not open storage database of Chrome!".to_string()))?;
		/*
		let mut iter = db.new_iter().unwrap();
		while let Some((key, value)) = iter.next()
		{
			let key = String::from_utf8(key).unwrap_or("?".to_string());
			let value = String::from_utf8(value).unwrap_or("?".to_string());
			println!("{key:?}: {value:?}");
		}
		*/
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
