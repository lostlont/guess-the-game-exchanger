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
		path::PathBuf,
	},
	configparser::ini::Ini,
	sqlite::Value,
	crate::browser::
	{
		Browser,
		Entry,
	},
};

pub struct Firefox
{
	db_path: PathBuf,
}

impl Firefox
{
	pub fn try_new() -> Result<Option<Box<dyn Browser>>, String>
	{
		let firefox_dir = get_firefox_dir()?;

		let result = if let Some(firefox_dir) = firefox_dir
		{
			let profiles_path = firefox_dir.join("profiles.ini");
			let ini = read_profiles_ini(&profiles_path)?;
			let profile_dir = get_profile_dir(&ini)?;
			let db_path = firefox_dir
				.join(&profile_dir)
				.join("storage/default/https+++guessthe.game/ls/data.sqlite");

			if !db_path.exists()
			{
				return Err("Could not find profiles file of Firefox!".to_string());
			}
			
			let firefox: Box<dyn Browser> = Box::new(Firefox
			{
				db_path,
			});

			Some(firefox)
		}
		else
		{
			None
		};

		Ok(result)
	}
}

impl Browser for Firefox
{
	fn name(&self) -> &str
	{
		"Firefox"
	}

	fn export(&self) -> Result<Vec<Entry>, String>
	{
		let connection = sqlite::open(&self.db_path)
			.or(Err("Could not open storage database file of Firefox!".to_string()))?;

		let mut statement = connection.prepare("select key, utf16_length, value from data")
			.or(Err("Could not prepare select statement on database of Firefox!".to_string()))?;

		let entries =  statement
			.iter()
			.collect::<Result<Vec<_>, _>>()
			.or(Err("Could not read a row from the database of Firefox!".to_string()))?
			.iter()
			.map(|row| Entry
				{
					key: row.read::<&str, _>(0).to_string(),
					utf16_length: row.read::<i64, _>(1),
					value: Vec::from(row.read::<&[u8], _>(2)),
				})
			.collect::<Vec<_>>();

		Ok(entries)
	}

	fn import(&self, entries: Vec<Entry>) -> Result<(), String>
	{
		let backup_path = self.db_path.with_extension("bck");
		fs::copy(&self.db_path, &backup_path)
			.or(Err("Could not back up storage database of Firefox!".to_string()))?;

		let connection = sqlite::open(&self.db_path)
			.or(Err("Could not open storage database file of Firefox!".to_string()))?;

		let mut delete_statement = connection.prepare("delete from data")
			.or(Err("Could not prepare delete statement on database of Firefox!".to_string()))?;

		delete_statement
			.iter()
			.collect::<Result<Vec<_>, _>>()
			.or(Err("Could not delete rows from the database of Firefox!".to_string()))?;

		for entry in entries
		{
			let mut insert_statement = connection.prepare("insert into data values (:key, :utf16_length, 1, 0, 0, :value)")
				.or(Err("Could not prepare insert statement on database of Firefox!".to_string()))?;

			let parameters = [
				(":key", entry.key.into()),
				(":utf16_length", entry.utf16_length.into()),
				(":value", entry.value.into()),
			];
			insert_statement.bind::<&[(_, Value)]>(&parameters[..])
				.or(Err("Could not bind parameters to statement in insert statement on database of Firefox!"))?;

			insert_statement
				.iter()
				.collect::<Result<Vec<_>, _>>()
				.or(Err("Could not insert rows into the database of Firefox!".to_string()))?;
		}

		Ok(())
	}
}

fn get_firefox_dir() -> Result<Option<PathBuf>, String>
{
	let data_dir = dirs::data_dir();
	let data_dir = data_dir
		.ok_or("No data directory is present!".to_string())?;
	let firefox_dir = data_dir.join("Mozilla/Firefox");

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

fn read_profiles_ini(profiles_path: &PathBuf) -> Result<Ini, String>
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
