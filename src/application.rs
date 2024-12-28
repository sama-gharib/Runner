use raylib::prelude::*;

use crate::ui::Ui;
use crate::game::Game;


pub struct Application {
	ui: Ui,
	game: Game
}

impl Application {
	pub fn new() -> Self {
		Self {
			ui: Default::default(),
			game: Game::new()
		}
	}

	pub fn run(&mut self) {
		let (mut rl, thread) = raylib::init()
			.title("Runner")
			.size(800, 450)
			.build();

		rl.set_target_fps(60);

		while !rl.window_should_close() && !self.ui.is_finished() {
			if let Some(state) = self.ui.state() {
				if state == "Play" { self.game.update(&mut rl); }
			}
			self.ui.update(&mut rl);

			let mut d = rl.begin_drawing(&thread);
			d.clear_background(Color::BLACK);
			if let Some(state) = self.ui.state() {
				if state == "Play" { self.game.draw(&mut d); }
			}
			self.ui.draw(&mut d);
			d.draw_fps(10, 10);
		}
	}
}