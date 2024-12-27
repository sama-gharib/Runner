use super::View;
use crate::model::Model;

use raylib::prelude::*;

pub struct Button {
	model: crate::model::button::Button,
	title: &'static str,
	background: Color,
	hovered: Color,
	pressed: Color,
	foreground: Color
}

fn color_lerp(a: Color, b: Color, f: f32) -> Color {
	Color::new(
		(a.r as f32 + (b.r as f32 - a.r as f32) * f) as u8,
		(a.g as f32 + (b.g as f32 - a.g as f32) * f) as u8,
		(a.b as f32 + (b.b as f32 - a.b as f32) * f) as u8,
		255
	)
}

impl View for Button {
	fn draw(&mut self, handle: &mut RaylibDrawHandle) {

		let pos = self.position();
		let siz = self.size();

		handle.draw_rectangle_v(pos, siz, if self.model.is_pressed() { color_lerp(self.pressed, self.hovered, self.model.is_hovered() as i32 as f32 / 2.0) } else if self.model.is_hovered() { self.hovered } else { self.background });
		handle.draw_text(self.title, pos.x as i32, pos.y as i32, siz.y as i32, if self.model.is_pressed() { self.background } else { self.foreground });
	}

	fn get_model(&mut self) -> &mut dyn Model {
		&mut self.model
	}
}

impl Button {
	const ZONE: Vector2 = Vector2 {x: 800.0, y: 450.0};

	pub fn from(model: crate::model::button::Button) -> Self {
		Self {
			model,
			title: "default",
			foreground: Color::BLACK,
			hovered: Color::GRAY,
			pressed: Color::new(200, 200, 200, 255),
			background: Color::WHITE
		}
	}

	pub fn title(mut self, title: &'static str) -> Self {
		self.title = title;
		self
	}

	pub fn foreground(mut self, foreground: Color) -> Self {
		self.foreground = foreground;
		self
	}

	pub fn hovered(mut self, hovered: Color) -> Self {
		self.hovered = hovered;
		self
	}

	pub fn pressed(mut self, pressed: Color) -> Self {
		self.pressed = pressed;
		self
	}

	pub fn background(mut self, background: Color) -> Self {
		self.background = background;
		self
	}

	pub fn position(&self) -> Vector2 {
		let mut position = self.model.get_center() - self.model.get_size() / 2.0;

		position.x *= Self::ZONE.x;
		position.y *= Self::ZONE.y;

		position
	}

	pub fn size(&self) -> Vector2 {
		let mut size = self.model.get_size();

		size.x *= Self::ZONE.x;
		size.y *= Self::ZONE.y;

		size
	}

	pub fn to_local(v: Vector2) -> Vector2 {
		Vector2::new(
			v.x / Self::ZONE.x,
			v.y / Self::ZONE.y
		)
	}
}