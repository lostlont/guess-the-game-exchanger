use std::path::Path;

mod firefox;

pub use firefox::Firefox;

pub trait Browser
{
	fn name(&self) -> &str;
	fn export(&self, path: &Path);
}
