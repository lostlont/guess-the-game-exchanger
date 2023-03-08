mod firefox;
pub use firefox::Firefox;

pub trait Browser
{
	fn name(&self) -> &str;
}
