use raylib::prelude::*;
use menu::Menu;

use widget::Button;
use std::cmp::Ordering;

mod menu;
mod widget;

pub struct Ui {
	menus: Vec::<Menu>,
	current_menu: usize,
	finished: bool
}

impl Default for Ui {
	fn default() -> Self {
		Ui::new()
			.add_menu(
				Menu::new("Main menu")
					.add_widget(Box::new(
						Button::new(Vector2::new(100., 100.), Vector2::new(200., 50.))
							.title("Play")
							.state_changer()
					))
					.add_widget(Box::new(
						Button::new(Vector2::new(100., 175.), Vector2::new(200., 50.))
							.title("Options")
							.state_changer()
					))
					.add_widget(Box::new(
						Button::new(Vector2::new(100., 250.), Vector2::new(200., 50.))
							.title("Quit")
							.state_changer()
					))
			)
			.add_menu(
				Menu::new("Play")
					.add_widget(Box::new(
						Button::new(Vector2::new(10., 10.), Vector2::new(200., 50.))
							.title("Main menu")
							.state_changer()
					))
			)
			.add_menu(
				Menu::new("Quit")
					.add_widget(Box::new(
						Button::new(Vector2::new(100., 100.), Vector2::new(200., 50.))
							.title("Yes")
							.window_destroyer()
					))
					.add_widget(Box::new(
						Button::new(Vector2::new(100., 175.), Vector2::new(200., 50.))
							.title("Main menu")
							.state_changer()
					))
			)
	}
}

impl Ui {
	pub const CHANGE_STATE_COMMAND: &str = "change_state:";
	pub const DESTROY_WINDOW_COMMAND: &str = "destroy:";

	pub fn new() -> Self {
		Self {
			menus: Vec::<Menu>::new(),
			current_menu: 0,
			finished: false
		}
	}

	pub fn add_menu(mut self, m: Menu) -> Self {
		self.menus.push(m);
		self
	}

	pub fn draw(&self, rl: &mut RaylibDrawHandle) {
		if let Some(menu) = self.menus.get(self.current_menu) {
			menu.draw(rl);
		}
	}

	pub fn update(&mut self, rl: &mut RaylibHandle) {
		if let Some(menu) = self.menus.get_mut(self.current_menu) {
			
			menu.update(rl);

			for activation in menu.activations() {
				if activation.starts_with(Self::CHANGE_STATE_COMMAND) {
					let next_menu: String = activation
						.chars()
						.skip(Self::CHANGE_STATE_COMMAND.len())
						.collect();

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
				} else if activation.starts_with(Self::DESTROY_WINDOW_COMMAND) {
					self.finished = true;
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
}