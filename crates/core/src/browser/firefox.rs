use
{
	std::
	{
		fs::File,
		io::Read,
		path::PathBuf,
	},
	configparser::ini::Ini,
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
	pub fn new() -> Result<Box<dyn Browser>, String>
	{
		let firefox_dir = get_firefox_dir()?;
		let profiles_path = firefox_dir.join("profiles.ini");
		let ini = read_profiles_ini(&profiles_path)?;
		let profile_dir = get_profile_dir(&ini)?;
		let db_path = firefox_dir
			.join(&profile_dir)
			.join("storage/default/https+++guessthe.game/ls/data.sqlite");
		
		Ok(Box::new(Firefox
		{
			db_path,
		}))
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
}

fn get_firefox_dir() -> Result<PathBuf, String>
{
	let config_dir = dirs::config_dir();
	let config_dir = config_dir
		.ok_or("No config directory is present!".to_string())?;
	let firefox_dir = config_dir.join("Mozilla/Firefox");

	Ok(firefox_dir)
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
