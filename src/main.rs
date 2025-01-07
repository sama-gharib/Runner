use application::Application;
use macroquad::prelude::*;

mod application;
mod ui;
mod game;

fn window_conf() -> Conf {
	Conf {
		window_width: 800,
		window_height: 450,
		window_title: String::from("Runner"),
		..Default::default()
	}
}

#[macroquad::main(window_conf)]
async fn main() {
	let mut application = Application::new();

	application.run().await;
}