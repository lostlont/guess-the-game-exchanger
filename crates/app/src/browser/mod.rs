use
{
	iced::
	{
		alignment::
		{
			Horizontal,
			Vertical,
		},
		widget::
		{
			button,
			row,
			text,
		},
		Element,
		Length,
	},
	crate::SPACING,
};

mod browser_message;
mod firefox;
pub use
{
	browser_message::BrowserMessage,
	firefox::Firefox,
};

pub trait Browser
{
	fn name(&self) -> &str;

	fn view(&self) -> Element<BrowserMessage>
	{
		let browser_name = self.name();
		let export_button = view_browser_button(&format!("Export from {browser_name}"), BrowserMessage::Export);
		let import_button = view_browser_button(&format!("Import into {browser_name}"), BrowserMessage::Import);
	
		row![export_button, import_button]
			.spacing(SPACING)
			.into()
	}
}

fn view_browser_button<'a>(button_text: &str, message: BrowserMessage) -> Element<'a, BrowserMessage>
{
	let button_text = text(button_text)
		.width(Length::Fill)
		.horizontal_alignment(Horizontal::Center)
		.vertical_alignment(Vertical::Center);
	let button = button(button_text)
		.width(Length::Fill)
		.height(32)
		.on_press(message);

	button.into()
}
