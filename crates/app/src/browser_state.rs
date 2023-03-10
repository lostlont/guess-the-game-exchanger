use
{
	core::browser::Browser,
	crate::
	{
		browser_message::BrowserMessage,
		browser_type::BrowserType,
	},
};

pub struct BrowserState
{
	pub browser_type: BrowserType,
	pub browser: Option<Box<dyn Browser>>,
	pub message: BrowserMessage,
}
