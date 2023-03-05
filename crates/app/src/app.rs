use
{
	iced::
	{
		alignment::
		{
			Alignment,
			Horizontal,
			Vertical,
		},
		widget::text,
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
			BrowserMessage,
		},
		app_message::AppMessage,
		PADDING,
		SPACING,
	},
};

pub struct App
{
	browsers: Vec<Result<Box<dyn Browser>, String>>,
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
			.map(|b| view_browser(b))
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

fn view_browser(browser: &Result<Box<dyn Browser>, String>) -> Element<BrowserMessage>
{
	match browser
	{
		Ok(browser) => browser.view(),
		Err(error) => text(error)
			.width(Length::Fill)
			.horizontal_alignment(Horizontal::Center)
			.vertical_alignment(Vertical::Center)
			.height(32)
			.into()
	}
}