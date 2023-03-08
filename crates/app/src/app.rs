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
		widget::
		{
			button,
			row,
			text,
		},
		Application,
		Command,
		Element,
		Length,
	},
	core::browser::
	{
		Browser,
		Firefox,
	},
	crate::
	{
		app_message::AppMessage,
		PADDING,
		SPACING,
	},
};

pub struct App
{
	browsers: Vec<Result<Box<dyn Browser>, String>>,
}

impl Application for App
{
	type Executor = iced::executor::Default;
	type Message = AppMessage;
	type Theme = iced::theme::Theme;
	type Flags = ();

	fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>)
	{
		let result = Self
		{
			browsers: vec![
				Firefox::new(),
			],
		};

		(result, Command::none())
	}

	fn title(&self) -> String
	{
		String::from("Guess The Game Exchanger")
	}

	fn update(&mut self, message: Self::Message) -> Command<Self::Message>
	{
		/*
		match message
		{
			AppMessage::BrowserMessage(bm) =>
			{
				match bm
				{
					BrowserMessage::Export =>
					{
						let path = rfd::FileDialog::new()
							.add_filter("JSON", &["json"])
							.save_file();

						if let Some(path) = path
						{
							println!("Picked file: {path:?}");
						}
					}
					_ =>
					{
					}
				}
			}
		}
		*/

		Command::none()
	}

	fn view(&self) -> Element<Self::Message>
	{
		let mut result = iced::widget::column![];
		let browser_views = self.browsers
			.iter()
			.map(|b| view_browser_or_error(b));
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

fn view_browser_or_error(browser: &Result<Box<dyn Browser>, String>) -> Element<AppMessage>
{
	match browser
	{
		Ok(browser) => view_browser(browser.as_ref()),
		Err(error) => text(error)
			.width(Length::Fill)
			.horizontal_alignment(Horizontal::Center)
			.vertical_alignment(Vertical::Center)
			.height(32)
			.into()
	}
}

fn view_browser(browser: &dyn Browser) -> Element<AppMessage>
{
	let browser_name = browser.name();
	let export_button = view_browser_button(&format!("Export from {browser_name}"));
	let import_button = view_browser_button(&format!("Import into {browser_name}"));

	row![export_button, import_button]
		.spacing(SPACING)
		.into()
}

fn view_browser_button<'a>(button_text: &str) -> Element<'a, AppMessage>
{
	let button_text = text(button_text)
		.width(Length::Fill)
		.horizontal_alignment(Horizontal::Center)
		.vertical_alignment(Vertical::Center);
	let button = button(button_text)
		.width(Length::Fill)
		.height(32);

	button.into()
}
