#[derive(Debug)]
pub struct Entry
{
	pub key: String,
	pub utf16_length: i64,
	pub value: Vec<u8>,
}
