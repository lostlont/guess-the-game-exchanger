use core::browser::Browser;
use crate::browser_type::BrowserType;

pub enum Component
{
	Browser
	{
		browser_type: BrowserType,
		browser: Box<dyn Browser>,
	},
	Error(String)
}
