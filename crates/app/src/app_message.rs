use crate::
{
	browser_type::BrowserType,
	command_type::CommandType,
};

#[derive(Clone, Debug)]
pub struct AppMessage
{
	pub browser_type: BrowserType,
	pub command_type: CommandType,
}
