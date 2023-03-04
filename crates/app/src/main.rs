use iced::
{
	Sandbox,
	Settings,
};

mod app;
mod message;
use app::App;

pub fn main() -> iced::Result
{
	let settings = Settings
	{
		window: iced::window::Settings
		{
			size: (320, 200),
			resizable: false,
			..Default::default()
		},
		..Default::default()
	};
	App::run(settings)
}
