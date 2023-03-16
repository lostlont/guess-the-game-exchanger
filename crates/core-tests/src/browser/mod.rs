mod chrome;
mod firefox;

#[cfg(test)]
pub mod tests
{
	pub const VALID_ENTRIES : [(&str, &str); 15] = [
		("255_gamestate", "win"),
		("255_guess1", "Game Name"),
		("300_gamestate", "lose"),
		("300_guess1", "Skipped!"),
		("300_guess2", "Skipped!"),
		("onefers", "1"),
		("twofers", "2"),
		("threefers", "3"),
		("quads", "4"),
		("fivers", "5"),
		("sixers", "6"),
		("currentstreak", "0"),
		("maxstreak", "3"),
		("totalplayed", "2"),
		("totalwon", "1"),
	];

	pub const INVALID_ENTRIES : [(&str, &str); 3] = [
		("_cmpRepromptHash", "ABC"),
		("CMPList", "{\"key\":\"value\"}"),
		("noniabvendorconsent", "consent"),
	];
}
