use
{
	iced::
	{
		alignment::Alignment,
		Element,
		Length,
		Sandbox,
	},
	crate::
	{
		browser::
		{
			self,
			Browser,
		},
		app_message::AppMessage,
		PADDING,
		SPACING,
	},
};

pub struct App
{
	browsers: Vec<Box<dyn Browser>>,
}

impl Sandbox for App
{
	type Message = AppMessage;

	fn new() -> Self
	{
		Self
		{
			browsers: vec![
				browser::Firefox::new(),
			],
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
			AppMessage::BrowserMessage(_) =>
			{
			}
		}
	}

	fn view(&self) -> Element<Self::Message>
	{
		let mut result = iced::widget::column![];
		let browser_views = self.browsers
			.iter()
			.map(|b| b.view())
			.map(|e| e.map(|m| AppMessage::BrowserMessage(m)));
		for view in browser_views
		{
			result = result.push(view);
		}
		result
			.width(Length::Fill)
			.align_items(Alignment::Center)
			.spacing(SPACING)
			.padding(PADDING)
			.into()
	}
}
