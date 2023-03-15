pub mod entry;
pub mod firefox;
pub mod profile;

use profile::Profile;

pub trait Browser
{
	fn name(&self) -> &str;
	fn export(&self) -> Result<Profile, String>;
	fn import(&self, profile: Profile) -> Result<(), String>;
}
