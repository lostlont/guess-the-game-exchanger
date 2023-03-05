use crate::browser::Browser;

pub struct Firefox
{
}

impl Firefox
{
	pub fn new() -> Result<Box<dyn Browser>, String>
	{
		let config_dir = dirs::config_dir();
		if let Some(config_dir) = config_dir
		{
			Ok(Box::new(Firefox
			{
			}))
		}
		else
		{
			Err("No config directory is present for Firefox!".to_string())
		}
		//File::open("")
		//let mut ini = Ini::new();
		//ini.read(ini_content);
	}
}

impl Browser for Firefox
{
	fn name(&self) -> &str
	{
		"Firefox"
	}
}
