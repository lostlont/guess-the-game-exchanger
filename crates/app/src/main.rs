use iced::
{
	Application,
	Settings,
};

mod app;
mod app_message;
mod browser_message;
mod browser_state;
mod browser_type;
mod command_type;
use app::App;

pub const PADDING: u16 = 8;
pub const SPACING: u16 = PADDING;

pub fn main() -> iced::Result
{
	let settings = Settings
	{
		window: iced::window::Settings
		{
			size: (360, 500),
			resizable: false,
			..Default::default()
		},
		..Default::default()
	};
	App::run(settings)
}
