use iced::
{
	Sandbox,
	Settings,
};

mod app;
mod browser;
mod message;
use app::App;

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
