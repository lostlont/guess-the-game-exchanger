use iced::{Element, Sandbox, Settings};

pub fn main() -> iced::Result
{
	Hello::run(Settings::default())
}

struct Hello;

impl Sandbox for Hello
{
	type Message = ();

	fn new() -> Hello
	{
		Hello
	}

	fn title(&self) -> String
	{
		String::from("Guess The Game Exchanger")
	}

	fn update(&mut self, _message: Self::Message)
	{
	}

	fn view(&self) -> Element<Self::Message>
	{
		"Hello, world!".into()
	}
}
