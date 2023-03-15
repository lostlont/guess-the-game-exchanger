use
{
	serde::
	{
		Deserialize,
		Serialize,
	},
	super::ProfileEntry,
};

#[derive(Deserialize, Serialize)]
pub struct Profile
{
	entries: Vec<ProfileEntry>,
}

impl Profile
{
	pub fn new() -> Self
	{
		Self
		{
			entries: vec![],
		}
	}

	pub fn from(entries: impl IntoIterator<Item = ProfileEntry>) -> Self
	{
		Self
		{
			entries: entries
				.into_iter()
				.collect(),
		}
	}

	pub fn get_entries(&self) -> impl Iterator<Item = &ProfileEntry>
	{
		self.entries.iter()
	}
}
