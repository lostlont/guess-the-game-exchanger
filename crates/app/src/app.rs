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
			CommandType::AskPathToExport =>
			{
				self.is_enabled = false;
				Command::perform(self.ask_path_to_export(message.browser_type), |m| m)
			}
			CommandType::Export(path) =>
			{
				self.is_enabled = true;
				let _bt = message.browser_type;
				let browser = self.components
					.iter()
					.find_map(|c| match c
					{
						Component::Browser{browser_type: _bt, browser} => Some(browser),
						Component::Error(_) => None,
					})
					.unwrap();

				browser
					.as_ref()
					.export(&path);

				Command::none()
			}
			CommandType::AskPathToImport =>
			{
				self.is_enabled = false;
				Command::perform(self.ask_path_to_import(message.browser_type), |m| m)
			}
			CommandType::Import(path) =>
			{
				self.is_enabled = true;
				Command::none()
			}
			CommandType::Cancel =>
			{
				self.is_enabled = true;
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
	fn ask_path_to_export(&self, browser_type: BrowserType) -> impl Future<Output = AppMessage> + 'static
	{
		async move
		{
			let path = rfd::AsyncFileDialog::new()
				.add_filter("JSON", &["json"])
				.save_file()
				.await;

			let command_type = if let Some(path) = path
			{
				CommandType::Export(path.into())
			}
			else
			{
				CommandType::Cancel
			};

			AppMessage
			{
				browser_type,
				command_type,
			}
		}
	}

	fn ask_path_to_import(&self, browser_type: BrowserType) -> impl Future<Output = AppMessage> + 'static
	{
		async move
		{
			let path = rfd::AsyncFileDialog::new()
				.add_filter("JSON", &["json"])
				.pick_file()
				.await;

			let command_type = if let Some(path) = path
			{
				CommandType::Import(path.into())
			}
			else
			{
				CommandType::Cancel
			};

			AppMessage
			{
				browser_type,
				command_type,
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
				command_type: CommandType::AskPathToExport,
			});

		let import_button = self.view_browser_button(
			&format!("Import into {browser_name}"),
			AppMessage
			{
				browser_type,
				command_type: CommandType::AskPathToImport,
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
