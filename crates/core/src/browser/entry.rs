use serde::
{
	Deserialize,
	Serialize,
};

#[derive(Debug, Deserialize, Serialize)]
pub struct Entry
{
	pub key: String,
	pub utf16_length: i64,
	pub value: Vec<u8>,
}
