use iced::
{
	Sandbox,
	Settings,
};

mod app;
mod app_message;
mod browser;
use app::App;

pub const PADDING: u16 = 8;
pub const SPACING: u16 = PADDING;

pub fn main() -> iced::Result
{
	let settings = Settings
	{
		window: iced::window::Settings
		{
			size: (480, 48),
			resizable: false,
			..Default::default()
		},
		..Default::default()
	};
	App::run(settings)
}
