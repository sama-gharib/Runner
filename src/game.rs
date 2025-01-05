//! In-game world

use world::World;
use raylib::prelude::*;
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
	pub fn new(to_load: &str, rm: &mut ResourceManager, rl: &mut RaylibHandle, thread: &RaylibThread) -> Self {
		Self {
			world: World::from((to_load, rm, rl, thread)),
			paused: false
		}
	}

	pub fn update(&mut self, rl: &mut RaylibHandle) {
		if !self.paused {
			self.world.update(rl);
		}

		if rl.is_key_pressed(KeyboardKey::KEY_ESCAPE) {
			self.paused = !self.paused;
		}
	}

	pub fn draw(&mut self, rl: &mut RaylibDrawHandle) {
		self.world.draw(rl);
		if self.paused {
			rl.clear_background(Color::new(0, 0, 0, 100));
		}
	}
}