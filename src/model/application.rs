use raylib::prelude::*;

pub enum ApplicationState {
	MainMenu,
	Game
}

pub struct ApplicationSettings {
	window_size: (i32, i32),
	max_fps: u32
}

impl Default for ApplicationSettings {
	fn default() -> Self {
		Self {
			window_size: (800, 450),
			max_fps: 60
		}
	}
}

pub struct Application {

	state: ApplicationState,
	settings: ApplicationSettings
}

impl Application {

	pub fn new() -> Self {
		Self {
			state: ApplicationState::MainMenu,
			settings: Default::default()
		}
	}

	pub fn run(&mut self) {
		let size = self.settings.window_size;

		let (mut rl, thread) = raylib::init()
			.size(size.0, size.1)
			.title("Runner")
			.build();

		rl.set_target_fps(self.settings.max_fps);

		while !rl.window_should_close() {
			let mut d = rl.begin_drawing(&thread);

			d.clear_background(Color::BLACK);
		}
	}
}