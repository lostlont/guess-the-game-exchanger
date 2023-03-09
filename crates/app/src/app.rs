use
{
	std::future::Future,
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
	is_enabled: bool,
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
			is_enabled: true,
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
				self.is_enabled = false;

				Command::perform(self.export(message.browser_type), |m| m)
			}
			CommandType::ExportFinished =>
			{
				self.is_enabled = true;
				println!("Export finished");
				Command::none()
			}
			_ =>
			{
				Command::none()
			}
		}
	}

	fn view(&self) -> Element<Self::Message>
	{
		let mut result = iced::widget::column![];
		let component_views = self.components
			.iter()
			.map(|c| self.view_component(c));
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

impl App
{
	fn export(&mut self, browser_type: BrowserType) -> impl Future<Output = AppMessage> + 'static
	{
		async move
		{
			let path = rfd::AsyncFileDialog::new()
				.add_filter("JSON", &["json"])
				.save_file()
				.await;

			if let Some(path) = path
			{
				println!("Picked file: {path:?}");
			}

			AppMessage
			{
				browser_type,
				command_type: CommandType::ExportFinished,
			}
		}
	}

	fn view_component(&self, component: &Component) -> Element<AppMessage>
	{
		match component
		{
			Component::Browser{browser_type, browser} => self.view_browser(*browser_type, browser.as_ref()),
			Component::Error(error) => text(error)
				.width(Length::Fill)
				.horizontal_alignment(Horizontal::Center)
				.vertical_alignment(Vertical::Center)
				.height(32)
				.into()
		}
	}

	fn view_browser(&self, browser_type: BrowserType, browser: &dyn Browser) -> Element<AppMessage>
	{
		let browser_name = browser.name();
		
		let export_button = self.view_browser_button(
			&format!("Export from {browser_name}"),
			AppMessage
			{
				browser_type,
				command_type: CommandType::Export,
			});

		let import_button = self.view_browser_button(
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

	fn view_browser_button<'a>(&self, button_text: &str, message: AppMessage) -> Element<'a, AppMessage>
	{
		let button_text = text(button_text)
			.width(Length::Fill)
			.horizontal_alignment(Horizontal::Center)
			.vertical_alignment(Vertical::Center);
		let button = button(button_text)
			.width(Length::Fill)
			.height(32);

		let button = if self.is_enabled
		{
			button.on_press(message)
		}
		else
		{
			button
		};

		button.into()
	}
}
