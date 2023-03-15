use
{
	serde::
	{
		Deserialize,
		Serialize,
	},
	crate::browser::entry::Entry,
};

#[derive(Deserialize, Serialize)]
pub struct Profile
{
	entries: Vec<Entry>,
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

	pub fn from(entries: impl IntoIterator<Item = Entry>) -> Self
	{
		Self
		{
			entries: entries
				.into_iter()
				.collect(),
		}
	}

	pub fn get_entries(&self) -> impl Iterator<Item = &Entry>
	{
		self.entries.iter()
	}
}
