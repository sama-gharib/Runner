//! All UI interactions

use macroquad::prelude::*;
use menu::Menu;

use widget::*;
use std::cmp::Ordering;

mod menu;
mod widget;

/// Manages a group of menus and comunicates user 
/// actions to application
pub struct Ui {
	menus: Vec::<Menu>,
	current_menu: usize,
	finished: bool,
	requested_level: Option<String>,
	requested_volume: Option<f32>
}


/// Default application UI, specialized for this project
impl Default for Ui {
	
	fn default() -> Self {
		
		let mut level_selection = Menu::new("Play");
		level_selection = level_selection.add_widget(
			Box::new(
				Button::new(Vec2::new(10., 10.), Vec2::new(200., 50.))
					.title("Main menu")
					.role(SpecialRole::StateChanger)
			)
		);
		for (i, path) in std::fs::read_dir("res/levels").unwrap().enumerate() {
			level_selection = level_selection.add_widget(
				Box::new(
					Button::new(
						Vec2::new(300., i as f32 * 55.),
						Vec2::new(200., 50.)
					)
					.title(path.unwrap().path().file_name().unwrap().to_str().unwrap())
					.role(SpecialRole::StateChanger)
					.role(SpecialRole::LevelSelector)
				)
			);
		}

		let center = vec2(screen_width(), screen_height())/2.;

		Ui::new()
			.add_menu(
				Menu::new("Main menu")
					.add_widget(Box::new(
						Button::new(Vec2::new(100., 100.), Vec2::new(200., 50.))
							.title("Play")
							.role(SpecialRole::StateChanger)
					))
					.add_widget(Box::new(
						Button::new(Vec2::new(100., 175.), Vec2::new(200., 50.))
							.title("Options")
							.role(SpecialRole::StateChanger)
					))
					.add_widget(Box::new(
						Button::new(Vec2::new(100., 250.), Vec2::new(200., 50.))
							.title("Quit")
							.role(SpecialRole::StateChanger)
					))
			)
			.add_menu(level_selection)
			.add_menu(
				Menu::new(".lvl")
					.add_widget(Box::new(
							Button::new(Vec2::new(10., 10.), Vec2::new(200., 50.))
								.title("Main menu")
								.role(SpecialRole::StateChanger)
					))
			)
			.add_menu(
				Menu::new("Options")
					.add_widget(Box::new(
							Button::new(Vec2::new(10., 10.), Vec2::new(200., 50.))
								.title("Main menu")
								.role(SpecialRole::StateChanger)
					))
					.add_widget(Box::new(
						SlideBar::new(vec2(center.x - 100., 100.), 200.)
							.title("Master volume")
							.role(SpecialRole::VolumeChanger)
					))
			)
			.add_menu(
				Menu::new("Quit")
					.add_widget(Box::new(
						Button::new(Vec2::new(100., 100.), Vec2::new(200., 50.))
							.title("Yes")
							.role(SpecialRole::WindowDestroyer)
					))
					.add_widget(Box::new(
						Button::new(Vec2::new(100., 175.), Vec2::new(200., 50.))
							.title("Main menu")
							.role(SpecialRole::StateChanger)
					))
			)
	}
}

impl Ui {

	pub fn new() -> Self {
		Self {
			menus: Vec::<Menu>::new(),
			current_menu: 0,
			finished: false,
			requested_level: None,
			requested_volume: None
		}
	}

	pub fn add_menu(mut self, m: Menu) -> Self {
		self.menus.push(m);
		self
	}

	/// Broadcasrs the draw call to current menu
	pub fn draw(&self) {
		if let Some(menu) = self.menus.get(self.current_menu) {
			menu.draw();
		}
	}

	/// Broadcasts the update call to current menu and manages menu
	/// switching and communicating user actions to application
	pub fn update(&mut self) {
		if let Some(menu) = self.menus.get_mut(self.current_menu) {
			
			menu.update();
			for (id, roles, activation) in menu.activations() {
				if roles.contains(&SpecialRole::StateChanger) {
					let mut next_menu: String = id
						.chars()
						.collect();

					if next_menu.contains('.') {
						next_menu = next_menu.chars().skip_while(|x| *x!='.').collect();
					}

					let next_menu_ids: Vec::<usize> = self.menus
						.iter()
						.enumerate()
						.filter_map(|(i, m)| if m.id() == next_menu { Some(i) } else { None })
						.collect();

					match next_menu_ids.len().cmp(&1usize) {
						Ordering::Less => eprintln!("Ui: No menu with id '{next_menu}'."),
						Ordering::Equal => self.current_menu = next_menu_ids[0],
						Ordering::Greater => eprintln!("Ui: Multiple menus with id '{next_menu}'.")
					}
				}
				if roles.contains(&SpecialRole::WindowDestroyer) {
					self.finished = true;
				}
				if roles.contains(&SpecialRole::LevelSelector) {
					self.requested_level = Some(id.clone());
				}
				if roles.contains(&SpecialRole::VolumeChanger) {
					self.requested_volume = Some(activation);
				}
			}
		} else {
			eprintln!("Ui: Menu id larger than menu Vec : {}.", self.current_menu);
		}
	}

	pub fn state(&self) -> Option<String> {
		if let Some(menu) = self.menus.get(self.current_menu) {
			Some(menu.id())
		} else {
			None
		}
	}

	pub fn is_finished(&self) -> bool { self.finished }

	pub fn get_requested_level(&mut self) -> Option<String> {
		std::mem::replace(&mut self.requested_level, None)
	}

	pub fn get_requested_volume(&mut self) -> Option<f32> {
		std::mem::replace(&mut self.requested_volume, None)
	}
}