mod entry;
mod firefox;

pub use entry::Entry;
pub use firefox::Firefox;

pub trait Browser
{
	fn name(&self) -> &str;
	fn export(&self) -> Result<Vec<Entry>, String>;
}
