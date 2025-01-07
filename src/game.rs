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
			clear_background(Color::new(0., 0., 0., 1.));
		}
	}
}