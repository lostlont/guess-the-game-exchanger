use
{
	std::fs::File,
	iced::
	{
		alignment::
		{
			Horizontal,
			Vertical,
		},
		widget::
		{
			self,
			button,
			row,
			text,
		},
		Alignment,
		Element,
		Length,
		Sandbox,
	},
	configparser::ini::Ini,
	crate::message::Message,
};

pub struct App
{
	is_exporting: bool,
}

const SPACING: u16 = 8;
const PADDING: u16 = SPACING;

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
			Message::ExportFromFirefox =>
			{
				let config_dir = dirs::config_dir();
				println!("Config dir: {config_dir:?}");
				//File::open("")
				//let mut ini = Ini::new();
				//ini.read(ini_content);
				self.is_exporting = true
			}
		}
	}

	fn view(&self) -> Element<Self::Message>
	{
		let firefox = App::view_firefox();
		widget::column![firefox]
			.width(Length::Fill)
			.align_items(Alignment::Center)
			.spacing(SPACING)
			.padding(PADDING)
			.into()
	}
}

impl App
{
	fn view_firefox<'a>() -> Element<'a, Message>
	{
		App::view_browser(
			"Firefox",
			Some(Message::ExportFromFirefox),
			None)
	}

	fn view_browser<'a>(browser_name: &str, export_message: Option<Message>, import_message: Option<Message>) -> Element<'a, Message>
	{
		let export_button = App::view_browser_button(&format!("Export from {browser_name}"), export_message);
		let import_button = App::view_browser_button(&format!("Import into {browser_name}"), import_message);

		row![export_button, import_button]
			.spacing(SPACING)
			.into()
	}

	fn view_browser_button<'a>(button_text: &str, message: Option<Message>) -> Element<'a, Message>
	{
		let button_text = text(button_text)
			.width(Length::Fill)
			.horizontal_alignment(Horizontal::Center)
			.vertical_alignment(Vertical::Center);
		let button = button(button_text)
			.width(Length::Fill)
			.height(32);

		let button = if let Some(message) = message
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
