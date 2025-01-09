//! Manages everything in-app

use macroquad::prelude::*;

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
	pub async fn run(&mut self) {

		let mut last_state = self.ui.state().unwrap();

		while !self.ui.is_finished() {
			let current_state = if let Some(state) = self.ui.state() {
				state
			} else {
				"Unknown state".to_string()
			};
			if let Some(level) = self.ui.get_requested_level() {
				self.game = Some(Game::new(&level, &mut self.resource_manager).await);
			} else if last_state != current_state {
				self.game = None;
			}

			last_state = self.ui.state().unwrap();

			if let Some(game) = &mut self.game {
				game.update();

				// Restart level
				if game.is_finished() && is_key_pressed(KeyCode::R) {
					game.reload(&mut self.resource_manager).await;
				}
			}
			
			self.ui.update();

			clear_background(BLACK);
				
			if let Some(game) = &mut self.game {
				game.draw();
				if game.is_finished() {
					draw_text("Press R to restart", 10., 440., 48., RED);
				}
			}
				
			self.ui.draw();
						
			next_frame().await;
		}
	} 
}