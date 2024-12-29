use world::World;
use raylib::prelude::*;

pub mod world;
mod object;

pub struct Game {
	world: World,
	paused: bool
}

impl Game {
	pub fn new() -> Self {
		Self {
			world: Default::default(),
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