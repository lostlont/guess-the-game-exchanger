pub mod chrome;
pub mod firefox;

use crate::profile::Profile;

pub trait Browser
{
	fn name(&self) -> &str;
	fn export(&self) -> Result<Profile, String>;
	fn import(&self, profile: Profile) -> Result<(), String>;
}
