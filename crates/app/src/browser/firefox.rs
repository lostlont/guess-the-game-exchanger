use crate::browser::Browser;

pub struct Firefox
{
	error: Option<String>,
}

impl Firefox
{
	pub fn new() -> Box<dyn Browser>
	{
		let config_dir = dirs::config_dir();
		if let Some(config_dir) = config_dir
		{
			Box::new(Firefox
			{
				error: None,
			})
		}
		else
		{
			Box::new(Firefox
			{
				error: Some("Could not find Firefox config directory!".to_string()),
			})
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

	fn error(&self) -> Option<&str>
	{
		self.error
			.as_ref()
			.map(|s| &s[..])
	}
}
