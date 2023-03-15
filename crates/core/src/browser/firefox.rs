use
{
	std::
	{
		fs::
		{
			self,
			File,
		},
		io::Read,
		path::
		{
			Path,
			PathBuf,
		},
	},
	configparser::ini::Ini,
	rusqlite::
	{
		named_params,
		Connection,
	},
	crate::profile::
	{
		Profile,
		ProfileEntry,
	},
	super::Browser,
};

pub struct Firefox<TDbProvider, TError>
where
	TDbProvider: Fn(&Path) -> Result<Connection, TError>,
{
	db_path: PathBuf,
	db_provider: TDbProvider,
	is_backup_enabled: bool,
}

impl<TDbProvider, TError> Firefox<TDbProvider, TError>
where
	TDbProvider: Fn(&Path) -> Result<Connection, TError> + 'static,
	TError: 'static,
{
	pub fn try_new<TIniProvider>(
		firefox_dir: &Path,
		ini_provider: TIniProvider,
		db_provider: TDbProvider,
		is_backup_enabled: bool)
		-> Result<Box<dyn Browser>, String>
	where
		TIniProvider: Fn(&Path) -> Result<Ini, String>,
	{
		let profiles_path = firefox_dir.join("profiles.ini");
		let ini = ini_provider(&profiles_path)?;
		let profile_dir = get_profile_dir(&ini)?;
		let db_path = firefox_dir
			.join(&profile_dir)
			.join("storage/default/https+++guessthe.game/ls/data.sqlite");

		let firefox: Box<dyn Browser> = Box::new(Firefox
		{
			db_path,
			db_provider,
			is_backup_enabled,
		});

		Ok(firefox)
	}
}

impl<TDbProvider, TError> Browser for Firefox<TDbProvider, TError>
where
	TDbProvider: Fn(&Path) -> Result<Connection, TError>,
{
	fn name(&self) -> &str
	{
		"Firefox"
	}

	fn export(&self) -> Result<Profile, String>
	{
		let connection = (self.db_provider)(&self.db_path)
			.or(Err("Could not open storage database file of Firefox!".to_string()))?;

		let mut statement = connection.prepare("select key, utf16_length, value from data")
			.or(Err("Could not prepare select statement on database of Firefox!".to_string()))?;

		let entries =  statement
			.query_map((), |row| Ok(ProfileEntry
				{
					key: row.get::<_, String>(0)?,
					utf16_length: row.get::<_, i64>(1)?,
					value: Vec::from(row.get::<_, Vec<u8>>(2)?),
				}))
			.or(Err("Could not query rows from the database of Firefox!".to_string()))?
			.collect::<Result<Vec<_>, _>>()
			.or(Err("Could not read a row from the database of Firefox!".to_string()))?
			.iter()
			.cloned()
			.collect::<Vec<_>>();

		let profile = Profile::from(entries);
		Ok(profile)
	}

	fn import(&self, profile: Profile) -> Result<(), String>
	{
		if self.is_backup_enabled
		{
			let backup_path = self.db_path.with_extension("bck");
			fs::copy(&self.db_path, &backup_path)
				.or(Err("Could not back up storage database of Firefox!".to_string()))?;
		}

		let connection = (self.db_provider)(&self.db_path)
			.or(Err("Could not open storage database file of Firefox!".to_string()))?;

		let mut delete_statement = connection.prepare("delete from data")
			.or(Err("Could not prepare delete statement on database of Firefox!".to_string()))?;

		delete_statement
			.execute([])
			.or(Err("Could not delete rows from the database of Firefox!".to_string()))?;

		for entry in profile.get_entries()
		{
			let mut insert_statement = connection.prepare("insert into data values (:key, :utf16_length, 1, 0, 0, :value)")
				.or(Err("Could not prepare insert statement on database of Firefox!".to_string()))?;

			insert_statement
				.execute(named_params!
					{
						":key": entry.key,
						":utf16_length": entry.utf16_length,
						":value": entry.value,
					})
				.or(Err("Could not execute insert statement on database of Firefox!"))?;
		}

		Ok(())
	}
}

pub fn get_firefox_dir() -> Result<Option<PathBuf>, String>
{
	let config_dir = dirs::config_dir();
	let config_dir = config_dir
		.ok_or("No config directory is present!".to_string())?;
	let firefox_dir = config_dir.join("Mozilla/Firefox");

	let result = if firefox_dir.exists()
	{
		Some(firefox_dir)
	}
	else
	{
		None
	};
	Ok(result)
}

pub fn read_profiles_ini(profiles_path: &Path) -> Result<Ini, String>
{
	let mut profiles_file = File::open(profiles_path)
		.or( Err("Could not open profiles file of Firefox!".to_string()))?;

	let mut profiles_content = String::new();
	profiles_file.read_to_string(&mut profiles_content)
		.or(Err("Failed to read content from profiles file of Firefox!"))?;

	let mut ini = Ini::new();
	ini.read(profiles_content)
		.or(Err("Failed to parse profiles file content as an ini file for Firefox!"))?;

	Ok(ini)
}

fn get_profile_dir(ini: &Ini) -> Result<String, String>
{
	let (_, install_section) = ini
		.get_map_ref()
		.iter()
		.filter(|(section, _)| section.starts_with("install"))
		.next()
		.ok_or("No Install section was found in the profiles of Firefox!".to_string())?;

	let default_entry = install_section
		.get("default")
		.ok_or("No Default key was found in the Install section of Firefox profiles!".to_string())?;

	let default_value = default_entry
		.clone()
		.ok_or("Default key has no value in the Install section of Firefox profiles!".to_string())?;

	Ok(default_value)
}
