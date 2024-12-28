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

		let mut last_state = self.ui.state().unwrap();

		while !rl.window_should_close() && !self.ui.is_finished() {
			let current_state = if let Some(state) = self.ui.state() {
				state
			} else {
				"Unknown state".to_string()
			};
			if last_state != current_state && last_state == "Play" {
				self.game = Game::new();
			}
			last_state = self.ui.state().unwrap();

			if current_state == "Play" { self.game.update(&mut rl); }
			
			self.ui.update(&mut rl);

			let mut d = rl.begin_drawing(&thread);
				d.clear_background(Color::BLACK);
				
				if current_state == "Play" { self.game.draw(&mut d); }
				
				self.ui.draw(&mut d);
				
				d.draw_fps(10, 10);
		}
	}
}