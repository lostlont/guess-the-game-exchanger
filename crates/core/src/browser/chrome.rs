use
{
	std::
	{
		fs,
		path::
		{
			Path,
			PathBuf,
		},
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

pub const KEY_PREFIX: &str = "_https://guessthe.game\0\u{1}";
pub const VALUE_PREFIX: &str = "\u{1}";
const ALLOWED_LIST: &[&str; 10] = &[
	"onefers", "twofers", "threefers", "quads", "fivers", "sixers",
	"currentstreak", "maxstreak", "totalplayed", "totalwon",
];

pub struct Chrome<TDbProvider, TError>
where
	TDbProvider: Fn(&Path) -> Result<rusty_leveldb::DB, TError>,
{
	db_dir: PathBuf,
	db_provider: TDbProvider,
	is_backup_enabled: bool,
}

impl<TDbProvider, TError> Chrome<TDbProvider, TError>
where
	TDbProvider: Fn(&Path) -> Result<rusty_leveldb::DB, TError> + 'static,
	TError: 'static,
{
	pub fn try_new(chrome_dir: &Path, db_provider: TDbProvider, is_backup_enabled: bool) -> Result<Box<dyn Browser>, String>
	{
		let db_dir = chrome_dir.join("User Data/Default/Local Storage/leveldb");

		let chrome: Box<dyn Browser> = Box::new(Chrome
		{
			db_dir,
			db_provider,
			is_backup_enabled,
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
			if let Some(key) = try_parse_key(key)?
			{
				if let Some(value) = try_parse_value(value)
				{
					let entry = ProfileEntry
					{
						key,
						value,
					};
					entries.push(entry);
				}
			}
		}

		let profile = Profile::from(entries);
		Ok(profile)
	}

	fn import(&self, profile: Profile) -> Result<(), String>
	{
		if self.is_backup_enabled
		{
			let backup_dir = self.db_dir.with_extension("bck");
			copy_dir(&self.db_dir, &backup_dir)
				.or(Err("Could not back up storage database of Chrome!".to_string()))?;
		}

		let mut db = (self.db_provider)(&self.db_dir)
			.or(Err("Could not open storage database of Chrome!".to_string()))?;

		let mut iter = db
			.new_iter()
			.or(Err("Could not iterate over database of Chrome!".to_string()))?;
		
		while let Some((key, _)) = iter.next()
		{
			if let Some(_) = try_parse_key(key.clone())?
			{
				db
					.delete(&key)
					.or(Err("Could not delete old entry in the database of Chrome!".to_string()))?;
			}
		}

		for entry in profile.get_entries()
		{
			let key = KEY_PREFIX.to_string() + &entry.key;
			let value = VALUE_PREFIX.to_string() + &entry.value;
			db
				.put(key.as_bytes(), value.as_bytes())
				.or(Err("Could not add entry to the database of Chrome!"))?;
		}

		Ok(())
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

fn copy_dir(source: &Path, destination: &Path) -> std::io::Result<()>
{
	fs::create_dir_all(destination)?;
	for entry in fs::read_dir(source)?
	{
		let entry = entry?;
		let path = entry.path();
		let destination_path = destination.join(entry.file_name());
		fs::copy(&path, &destination_path)?;
	}
	Ok(())
}

fn try_parse_key(key: Vec<u8>) -> Result<Option<String>, String>
{
	let key = String::from_utf8(key)
		.or(Err("Could not parse a key as a string from the database of Chrome!".to_string()))?;

	if let Some(key) = key.strip_prefix(KEY_PREFIX)
	{
		if key.ends_with("_gamestate") || key.contains("_guess") || ALLOWED_LIST.contains(&key)
		{
			return Ok(Some(key.to_string()));
		}
	}

	return Ok(None);
}

fn try_parse_value(value: Vec<u8>) -> Option<String>
{
	let value = String::from_utf8(value);

	if let Ok(value) = value
	{
		if let Some(value) = value.strip_prefix(VALUE_PREFIX)
		{
			return Some(value.to_string());
		}
	}

	return None;
}
