use raylib::prelude::*;

use crate::ui::Ui;
use crate::game::Game;


pub struct Application {
	ui: Ui,
	game: Option<Game>
}

impl Application {
	pub fn new() -> Self {
		Self {
			ui: Ui::new(),
			game: None
		}
	}

	pub fn run(&mut self) {
		let (mut rl, thread) = raylib::init()
			.title("Runner")
			.size(800, 450)
			.build();

		rl.set_target_fps(60);

		while !rl.window_should_close() {
			let mut d = rl.begin_drawing(&thread);

			d.clear_background(Color::BLACK);

			d.draw_fps(10, 10);
		}
	}
}