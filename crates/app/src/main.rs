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
	App::run(Settings::default())
}
