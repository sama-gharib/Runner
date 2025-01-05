//! Manages everything in-app

use raylib::prelude::*;

use crate::ui::Ui;
use crate::game::Game;
use crate::game::resource_manager::*;


/// Highest structure in the "ownership tree" of this project
/// Manages everything
pub struct Application {
	ui: Ui,
	game: Option<Game>,
	resource_manager: ResourceManager
}

impl Application {
	pub fn new() -> Self {
		Self {
			ui: Default::default(),
			game: None,
			resource_manager: ResourceManager::new()
		}
	}

	/// Main loop
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
			if let Some(level) = self.ui.get_requested_level() {
				self.game = Some(Game::new(&level, &mut self.resource_manager, &mut rl, &thread));
			} else if last_state != current_state {
				self.game = None;
			}

			last_state = self.ui.state().unwrap();

			if let Some(game) = &mut self.game { game.update(&mut rl); }
			
			self.ui.update(&mut rl);

			let mut d = rl.begin_drawing(&thread);
				d.clear_background(Color::BLACK);
				
				if let Some(game) = &mut self.game { game.draw(&mut d); }
				
				self.ui.draw(&mut d);
				
				d.draw_fps(10, 10);
		}
	}
}