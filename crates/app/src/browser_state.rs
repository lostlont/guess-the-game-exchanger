use core::browser::Browser;
use crate::browser_type::BrowserType;

pub struct BrowserState
{
	pub browser_type: BrowserType,
	pub browser: Option<Box<dyn Browser>>,
	pub message: Option<String>,
}
