//! In-game world

use world::World;
use macroquad::prelude::*;
use resource_manager::*;

pub mod world;
pub mod resource_manager;
pub mod animation;

mod object;

/// Manages the world to application behaviour
pub struct Game {
	world: World,
	paused: bool
}

impl Game {
	pub async fn new(to_load: &str, rm: &mut ResourceManager) -> Self {
		Self {
			world: World::from((to_load, rm)).await,
			paused: false
		}
	}

	pub fn update(&mut self) {
		if !self.paused {
			self.world.update();
		}

		if is_key_pressed(KeyCode::Escape) {
			self.paused = !self.paused;
		}
	}

	pub fn draw(&mut self) {
		self.world.draw();
		if self.paused {
			draw_rectangle(
				0.,
				0.,
				screen_width(),
				screen_height() as f32,
				Color::new(0., 0., 0., 0.5)
			);

			draw_rectangle(
				screen_width() * 0.45,
				screen_height() * 0.35,
				screen_width() * 0.04,
				screen_height() * 0.3,
				WHITE
			);

			draw_rectangle(
				screen_width() * 0.51,
				screen_height() * 0.35,
				screen_width() * 0.04,
				screen_height() * 0.3,
				WHITE
			);
		}
	}
}