use crate::model::{Model, Notification, button::{Menu, Button}};
use raylib::prelude::*;

#[derive(Debug, Clone, Copy)]
pub enum ApplicationState {
	MainMenu,
	Game,
	ConfirmQuit
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
	settings: ApplicationSettings,
	main_menu: Menu
}

impl Model for Application {
	fn notify(&mut self, n: Notification) {
		for (id, b) in self.main_menu.iter_mut() {
			b.notify(n);
			
			if b.check() {
				match *id {
					"play" => self.state = ApplicationState::Game,
					"quit" => self.state = ApplicationState::ConfirmQuit,
					_ => eprintln!("Unknown button id : '{id}'.")
				}
			}
		}
	}
}

impl Application {

	pub fn new() -> Self {
		let mut r = Self {
			state: ApplicationState::MainMenu,
			settings: Default::default(),
			main_menu: Menu::new()
		};
		r.main_menu.insert("play", Button::new(Vector2::new(0.5, 0.2), Vector2::new(0.3, 0.1)));
		r.main_menu.insert("quit", Button::new(Vector2::new(0.5, 0.4), Vector2::new(0.3, 0.1)));

		r
	}

	pub fn main_menu(&self) -> Vec::<Button> { self.main_menu.iter().map(|(_, x)| *x).collect() }
	pub fn state(&self) -> ApplicationState { self.state } 
}