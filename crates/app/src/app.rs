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
		browser_type::BrowserType,
		command_type::CommandType,
		component::Component,
		PADDING,
		SPACING,
	},
};

pub struct App
{
	components: Vec<Component>,
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
			components: vec![
				Firefox::new()
					.map_or_else(
						|error| Component::Error(error),
						|result| Component::Browser
						{
							browser_type: BrowserType::Firefox,
							browser: result,
						}),
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
		match message.command_type
		{
			CommandType::Export =>
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

		Command::none()
	}

	fn view(&self) -> Element<Self::Message>
	{
		let mut result = iced::widget::column![];
		let component_views = self.components
			.iter()
			.map(|c| view_component(c));
		for view in component_views
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

fn view_component(component: &Component) -> Element<AppMessage>
{
	match component
	{
		Component::Browser{browser_type, browser} => view_browser(*browser_type, browser.as_ref()),
		Component::Error(error) => text(error)
			.width(Length::Fill)
			.horizontal_alignment(Horizontal::Center)
			.vertical_alignment(Vertical::Center)
			.height(32)
			.into()
	}
}

fn view_browser(browser_type: BrowserType, browser: &dyn Browser) -> Element<AppMessage>
{
	let browser_name = browser.name();
	
	let export_button = view_browser_button(
		&format!("Export from {browser_name}"),
		AppMessage
		{
			browser_type,
			command_type: CommandType::Export,
		});

	let import_button = view_browser_button(
		&format!("Import into {browser_name}"),
		AppMessage
		{
			browser_type,
			command_type: CommandType::Import,
		});

	row![export_button, import_button]
		.spacing(SPACING)
		.into()
}

fn view_browser_button<'a>(button_text: &str, message: AppMessage) -> Element<'a, AppMessage>
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
