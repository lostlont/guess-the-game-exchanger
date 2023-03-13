pub mod entry;
pub mod firefox;

use entry::Entry;

pub trait Browser
{
	fn name(&self) -> &str;
	fn export(&self) -> Result<Vec<Entry>, String>;
	fn import(&self, entries: Vec<Entry>) -> Result<(), String>;
}
