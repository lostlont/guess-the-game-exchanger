use iced::
{
	alignment,
	Element,
	Length,
	Sandbox,
	widget::
	{
		button,
		self,
		text,
	},
};
use crate::message::Message;

pub struct App
{
	is_exporting: bool,
}

impl Sandbox for App
{
	type Message = Message;

	fn new() -> Self
	{
		Self
		{
			is_exporting: false,
		}
	}

	fn title(&self) -> String
	{
		String::from("Guess The Game Exchanger")
	}

	fn update(&mut self, message: Self::Message)
	{
		match message
		{
			Message::ExportFromFirefox => self.is_exporting = true,
		}
	}

	fn view(&self) -> Element<Self::Message>
	{
		let button = button("Export from Firefox")
			.on_press(Message::ExportFromFirefox);

		let label = text("Exporting...")
			.width(Length::Fill)
			.size(36)
			.horizontal_alignment(alignment::Horizontal::Center);

		if self.is_exporting
		{
			widget::column![button, label]
				.into()
		}
		else
		{
			button.into()            
		}
	}
}
