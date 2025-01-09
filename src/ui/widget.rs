//! Base elements of UI

use macroquad::prelude::*;


/// Represents any widget such as buttons, labels or even input fields
pub trait Widget {
	fn get_position(&self) -> Vec2;
	fn get_size(&self) -> Vec2;
	fn activation_factor(&mut self) -> f32;
	fn get_id(&self) -> String;
	fn update(&mut self);
	fn draw(&self);
	fn get_roles(&self) -> Vec::<SpecialRole>;
	fn contains(&self, v: Vec2) -> bool {
		   self.get_position().x < v.x && self.get_position().x + self.get_size().x > v.x
		&& self.get_position().y < v.y && self.get_position().y + self.get_size().y > v.y
	}
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SpecialRole {
	StateChanger,
	WindowDestroyer,
	LevelSelector,
	VolumeChanger
}

pub struct SlideBar {
	position: Vec2,
	length: f32,
	title: String,

	progress: f32,

	hovered: bool,
	held: bool,
	changed: bool,

	roles: Vec<SpecialRole>
}

impl SlideBar {
	const HEIGHT: f32 = 20.;

	pub fn new(position: Vec2, length: f32) -> Self {
		Self {
			position,
			length,
			title: String::new(),
			progress: 0.5,
			hovered: false,
			held: false,
			changed: false,
			roles: Default::default()
		}
	}

	pub fn title(mut self, s: &str) -> Self {
		self.title = s.to_owned();
		self
	}

	pub fn role(mut self, r: SpecialRole) -> Self {
		self.roles.push(r);
		self
	}
}

impl Widget for SlideBar {
	fn get_position(&self) -> Vec2 { self.position }
	fn get_size(&self) -> Vec2 { vec2(self.length, Self::HEIGHT) }
	fn activation_factor(&mut self) -> f32 { 
		self.changed as i32 as f32 * self.progress
	}
	fn get_id(&self) -> String { self.title.clone() }
	fn update(&mut self) {
		self.changed = false;

		let mouse = mouse_position();
		let mouse = vec2(mouse.0, mouse.1);
		let mouse_in = self.contains(mouse);

		if is_mouse_button_pressed(MouseButton::Left) && mouse_in {
			self.held = true;
		} else if self.held && is_mouse_button_released(MouseButton::Left) {
			self.held = false;
		}
				
		self.hovered = !self.held && mouse_in;

		if self.held {
			let before = self.progress;
			self.progress = (mouse.x - self.position.x) / self.length;
			self.progress = self.progress.max(0.).min(1.);
			self.changed = before != self.progress;
		}
	}
	fn draw(&self) {
		draw_rectangle(
			self.get_position().x,
			self.get_position().y,
			self.get_size().x,
			self.get_size().y,
			WHITE
		);
		draw_rectangle(
			self.get_position().x,
			self.get_position().y,
			self.get_size().x * self.progress,
			self.get_size().y,
			if self.hovered || self.held { GREEN } else { GRAY }
		);

		let font_size = self.get_size().y * 0.9;
		let text_width = measure_text(&self.title, None, font_size as u16, 1.).width;
		draw_text(
			&self.title,
			self.get_position().x + (self.get_size().x - text_width) / 2.,
			self.get_position().y + font_size,
			font_size,
			BLACK
		);
	}
	fn get_roles(&self) -> Vec::<SpecialRole> { self.roles.clone() }
}

pub struct Button {
	position: Vec2,
	size: Vec2,
	title: String,
	activated: f32,
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
	fn activation_factor(&mut self) -> f32 {
		std::mem::replace(&mut self.activated, 0.)
	}
	fn get_id(&self) -> String {
		self.title.clone()
	}
	fn update(&mut self) {
		let mouse = Vec2::from(mouse_position());

		let p = is_mouse_button_pressed(MouseButton::Left);
		let r = is_mouse_button_released(MouseButton::Left);

		self.hovered = self.contains(mouse);
		self.activated = (self.pressed && self.hovered && r) as i32 as f32;
		self.pressed = self.hovered && p || (self.pressed && self.activated == 0. && !r);
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
			activated: 0.,
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
}