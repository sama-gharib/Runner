use crate::model::{Model, Notification};
use std::collections::HashMap;
use raylib::prelude::*;

#[derive(Debug, Clone, Copy)]
pub struct Button {

	center: Vector2,
	size: Vector2,
	hovered: bool,
	pressed: bool,
	activated: bool
}

pub type Menu = HashMap::<&'static str, Button>;

impl Model for Button {
	fn notify(&mut self, n: Notification) {

		match n {
			Notification::MousePress (x, y) => if self.contains(crate::view::button::Button::to_local(Vector2::from((x, y)))) { self.pressed = true },
			Notification::MouseRelease (x, y) => {
				if self.contains(crate::view::button::Button::to_local(Vector2::from((x, y)))) {
					self.activated = true
				}
				self.pressed = false;
			},
			Notification::MouseMove (x, y) => if self.contains(crate::view::button::Button::to_local(Vector2::from((x, y)))) { self.hovered = true } else { self.hovered = false }
			_ => todo!()
		}
	}
}

impl Button {
	pub fn new(center: Vector2, size: Vector2) -> Self  {
		Self {
			center,
			size,
			hovered: false,
			pressed: false,
			activated: false
		}
	}

	pub fn check(&mut self) -> bool {
		std::mem::replace(&mut self.activated, false)
	}

	fn contains(&self, v: Vector2) -> bool {
		v.x > self.center.x - self.size.x / 2.0
		&& v.x < self.center.x + self.size.x / 2.0
		&& v.y > self.center.y - self.size.y / 2.0
		&& v.y < self.center.y + self.size.y / 2.0
	}

	pub fn is_hovered(&self) -> bool { self.hovered }
	pub fn is_activated(&self) -> bool  { self.activated }
	pub fn is_pressed(&self) -> bool { self.pressed }

	pub fn get_center(&self) -> Vector2 { self.center }
	pub fn get_size(&self) -> Vector2 { self.size }
}