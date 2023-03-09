#[derive(Clone, Debug)]
pub enum CommandType
{
	Export,
	Import,
	ExportFinished,
	ImportFinished,
}
