use application::Application;

mod application;
mod ui;
mod game;

fn main() {
	let mut application = Application::new();

	application.run();
}