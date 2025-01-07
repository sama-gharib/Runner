//! Base elements of UI

use macroquad::prelude::*;


/// Represents any widget such as buttons, labels or even input fields
pub trait Widget {
	fn get_position(&self) -> Vec2;
	fn get_size(&self) -> Vec2;
	fn is_activated(&mut self) -> bool;
	fn get_id(&self) -> String;
	fn update(&mut self);
	fn draw(&self);
	fn get_roles(&self) -> Vec::<SpecialRole>;
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum SpecialRole {
	StateChanger,
	WindowDestroyer,
	LevelSelector
}

pub struct Button {
	position: Vec2,
	size: Vec2,
	title: String,
	activated: bool,
	hovered: bool,
	pressed: bool,
	roles: Vec::<SpecialRole>
}

impl Widget for Button {
	fn get_position(&self) -> Vec2 {
		self.position
	}
	fn get_size(&self) -> Vec2 {
		self.size
	}
	fn is_activated(&mut self) -> bool {
		std::mem::replace(&mut self.activated, false)
	}
	fn get_id(&self) -> String {
		self.title.clone()
	}
	fn update(&mut self) {
		let mouse = Vec2::from(mouse_position());

		let p = is_mouse_button_pressed(MouseButton::Left);
		let r = is_mouse_button_released(MouseButton::Left);

		self.hovered = self.contains(mouse);
		self.activated = self.pressed && self.hovered && r;
		self.pressed = self.hovered && p || (self.pressed && !self.activated && !r);
	}

	fn draw(&self) {
		draw_rectangle(
			self.position.x,
			self.position.y,
			self.size.x,
			self.size.y,
			if self.pressed {
				Color::new(0.75, 0.75, 0.75, 1.)
			} else if self.hovered {
				GRAY
			} else {
				WHITE
			}
		);
		draw_text(
			&self.title.chars().take_while(|x| *x!='.')
				.collect::<String>(),
			self.position.x + 10.,
			self.position.y + self.size.y * 0.8,
			self.size.y * 0.6,
			BLACK
		);
	}

	fn get_roles(&self) -> Vec::<SpecialRole> {
 		self.roles.clone()
 	}
}

impl Button {
	pub fn new(position: Vec2, size: Vec2) -> Self {
		Self {
			position,
			size,
			title: String::new(),
			activated: false,
			hovered: false,
			pressed: false,
			roles: Default::default()
		}
	}

	pub fn title(mut self, x: &str) -> Self {
		self.title = x.to_string();
		self
	}

	pub fn role(mut self, r: SpecialRole) -> Self {
		self.roles.push(r);
		self
	}

	pub fn contains(&self, v: Vec2) -> bool {
		   self.position.x < v.x && self.position.x + self.size.x > v.x
		&& self.position.y < v.y && self.position.y + self.size.y > v.y
	}
}