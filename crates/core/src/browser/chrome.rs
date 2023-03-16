use
{
	std::path::
	{
		Path,
		PathBuf,
	},
	rusty_leveldb::LdbIterator,
	crate::
	{
		browser::Browser,
		profile::
		{
			Profile,
			ProfileEntry,
		},
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

		let mut iter = db
			.new_iter()
			.or(Err("Could not iterate over database of Chrome!".to_string()))?;
		let mut entries = vec![];
		while let Some((key, value)) = iter.next()
		{
			let key = String::from_utf8(key)
				.or(Err("Could not parse a key as a string from the database of Chrome!".to_string()))?;

			if let Some(key) = key.strip_prefix("_https://guessthe.game\0\u{1}")
			{
				let exclude_list = vec!["CMPList", "_cmpRepromptHash", "noniabvendorconsent"];
				if !exclude_list.contains(&key)
				{
					let value = String::from_utf8(value);

					if let Ok(value) = value
					{
						if let Some(value) = value.strip_prefix("\u{1}")
						{
							let entry = ProfileEntry
							{
								key: key.to_string(),
								value: value.to_string(),
							};
							entries.push(entry);
						}
					}
				}
			}
		}

		let profile = Profile::from(entries);
		Ok(profile)
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
