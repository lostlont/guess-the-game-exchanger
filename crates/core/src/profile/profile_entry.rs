use serde::
{
	Deserialize,
	Serialize,
};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct ProfileEntry
{
	pub key: String,
	pub value: String,
}
