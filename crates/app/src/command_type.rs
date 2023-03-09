use std::path::PathBuf;

#[derive(Clone, Debug)]
pub enum CommandType
{
	AskPathToExport,
	AskPathToImport,
	Export(PathBuf),
	Import(PathBuf),
	Cancel,
}
