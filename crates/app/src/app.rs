use
{
	std::
	{
		fs::File,
		future::Future,
		path::Path,
	},
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
		Color,
		Command,
		Element,
		Length,
	},
	rusqlite,
	core::browser::
	{
		chrome::
		{
			self,
			Chrome,
		},
		firefox::
		{
			self,
			Firefox,
		},
		Browser,
	},
	crate::
	{
		app_message::AppMessage,
		browser_message::BrowserMessage,
		browser_state::BrowserState,
		browser_type::BrowserType,
		command_type::CommandType,
		PADDING,
		SPACING,
	},
};

pub struct App
{
	browser_states: Vec<BrowserState>,
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
		let browsers = vec![
			(BrowserType::Firefox, App::try_new_firefox()),
			(BrowserType::Chrome, App::try_new_chrome()),
		];
		let browser_states = browsers
			.into_iter()
			.filter_map(|(browser_type, browser)| match browser
				{
					Ok(None) => None,
					Ok(Some(browser)) => Some(BrowserState
					{
						browser_type,
						browser: Some(browser),
						message: BrowserMessage::Empty,
					}),
					Err(error) => Some(BrowserState
					{
						browser_type,
						browser: None,
						message: BrowserMessage::Failure(error),
					}),
				})
			.collect();

		let result = Self
		{
			browser_states,
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
				self.set_browser_message(message.browser_type, BrowserMessage::Empty);
				Command::perform(self.ask_path_to_export(message.browser_type), |m| m)
			}
			CommandType::Export(path) =>
			{
				let browser = self.get_browser(message.browser_type);
				let export = self.export(browser, &path);
				let export_message = match export
				{
					Ok(()) => BrowserMessage::Success("Success!".to_string()),
					Err(error) => BrowserMessage::Failure(error),
				};
				self.set_browser_message(message.browser_type, export_message);
				self.is_enabled = true;
				Command::none()
			}
			CommandType::AskPathToImport =>
			{
				self.is_enabled = false;
				self.set_browser_message(message.browser_type, BrowserMessage::Empty);
				Command::perform(self.ask_path_to_import(message.browser_type), |m| m)
			}
			CommandType::Import(path) =>
			{
				let browser = self.get_browser(message.browser_type);
				let import = self.import(browser, &path);
				let import_message = match import
				{
					Ok(()) => BrowserMessage::Success("Success!".to_string()),
					Err(error) => BrowserMessage::Failure(error),
				};
				self.set_browser_message(message.browser_type, import_message);
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
		let title = text("Guess The Game Exchanger")
			.width(Length::Fill)
			.size(28)
			.horizontal_alignment(Horizontal::Center);

		let create_gap = || row![].height(SPACING / 2);

		let mut result = iced::widget::column![
			create_gap(),
			title,
			create_gap(),
		];

		for view in self.browser_states
			.iter()
			.map(|bs| self.view_browser_state(bs))
		{
			let view = iced::widget::container(view)
				.height(Length::Fill)
				.center_y();
			result = result.push(view);
		}

		result
			.width(Length::Fill)
			.height(Length::Fill)
			.align_items(Alignment::Center)
			.spacing(SPACING * 4)
			.padding(PADDING * 4)
			.into()
	}
}

impl App
{
	fn try_new_firefox() -> Result<Option<Box<dyn Browser>>, String>
	{
		let firefox_dir = firefox::get_firefox_dir()?;
		match firefox_dir
		{
			None => Ok(None),
			Some(firefox_dir) =>
			{
				let firefox = Firefox::try_new(
					&firefox_dir,
					firefox::read_profiles_ini,
					|p| rusqlite::Connection::open(p),
					true)?;
				Ok(Some(firefox))
			},
		}
	}

	fn try_new_chrome() -> Result<Option<Box<dyn Browser>>, String>
	{
		let chrome_dir = chrome::get_chrome_dir()?;
		match chrome_dir
		{
			None => Ok(None),
			Some(chrome_dir) =>
			{
				let chrome = Chrome::try_new(
					&chrome_dir,
					|p|
					{
						let options = rusty_leveldb::Options
						{
							create_if_missing: false,
							..rusty_leveldb::Options::default()
						};
						rusty_leveldb::DB::open(p, options)
					},
					true)?;
				Ok(Some(chrome))
			},
		}
	}
	
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

	fn get_browser(&self, browser_type: BrowserType) -> &dyn Browser
	{
		self.browser_states
			.iter()
			.find_map(|bs| if bs.browser_type == browser_type { Some(bs.browser.as_ref().unwrap()) } else { None })
			.unwrap()
			.as_ref()
	}

	fn set_browser_message(&mut self, browser_type: BrowserType, message: BrowserMessage)
	{
		let mut browser_state = self.browser_states
			.iter_mut()
			.find(|bs| bs.browser_type == browser_type)
			.unwrap();

		browser_state.message = message;
	}

	fn export(&self, browser: &dyn Browser, path: &Path) -> Result<(), String>
	{
		let profile = browser.export()?;

		let file = File::create(path)
			.or(Err("Could not create file for exporting at the selected path!".to_string()))?;
		
		serde_json::to_writer_pretty(file, &profile)
			.or(Err("Could not write JSON to the selected file!".to_string()))?;

		Ok(())
	}

	fn import(&self, browser: &dyn Browser, path: &Path) -> Result<(), String>
	{
		let file = File::open(path)
			.or(Err("Could not open file for importing at the selected path!".to_string()))?;

		let entries = serde_json::from_reader(file)
			.or(Err("Could not read JSON from the selected file!".to_string()))?;

		browser.import(entries)?;

		Ok(())
	}

	fn view_browser_state(&self, browser_state: &BrowserState) -> Element<AppMessage>
	{
		let mut result = iced::widget::column![];

		if let Some(browser) = &browser_state.browser
		{
			result = result.push(self.view_browser(browser_state.browser_type, browser.as_ref()));
		}

		result = result.push(self.view_message(&browser_state.message));

		result.into()
	}

	fn view_browser(&self, browser_type: BrowserType, browser: &dyn Browser) -> Element<AppMessage>
	{
		let browser_name = text(browser.name())
			.width(Length::Fill)
			.size(20)
			.horizontal_alignment(Horizontal::Center);
		
		let export_button = self.view_browser_button(
			"Export",
			AppMessage
			{
				browser_type,
				command_type: CommandType::AskPathToExport,
			});

		let import_button = self.view_browser_button(
			"Import",
			AppMessage
			{
				browser_type,
				command_type: CommandType::AskPathToImport,
			});

		iced::widget::column![
			browser_name,
			row![export_button, import_button].spacing(SPACING),
		]
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

	fn view_message(&self, message: &BrowserMessage) -> Element<AppMessage>
	{
		let content = match message
		{
			BrowserMessage::Empty => "",
			BrowserMessage::Success(message) => message,
			BrowserMessage::Failure(message) => message,
		};
		let color = match message
		{
			BrowserMessage::Failure(_) => Color::from_rgb8(255, 0, 0),
			_ => Color::from_rgb8(0, 0, 0),
		};
		text(content)
			.width(Length::Fill)
			.horizontal_alignment(Horizontal::Center)
			.vertical_alignment(Vertical::Center)
			.height(48)
			.style(color)
			.into()
	}
}
