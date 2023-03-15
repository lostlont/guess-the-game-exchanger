use serde::
{
	Deserialize,
	Serialize,
};

// TODO Rename to ProfileEntry
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Entry
{
	pub key: String,
	pub utf16_length: i64,
	pub value: Vec<u8>,
}
