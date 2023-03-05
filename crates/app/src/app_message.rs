use crate::browser::BrowserMessage;

#[derive(Clone, Debug)]
pub enum AppMessage
{
	BrowserMessage(BrowserMessage),
}
