//! Animation utilities

use std::rc::Rc;
use raylib::prelude::*;

use super::resource_manager::*;

#[derive(Clone, Debug)]
pub struct Animation {
	spritesheet: Rc::<Resource>,
	id: u32,
	length: u32,

	sustain: u32,
	looped: bool,

	sustain_countdown: u32,
	current_frame: u32

}

impl Animation {
	const UNIT: Vector2 = Vector2 { x: 32., y: 32. };

	pub fn new(path: &'static str, id: u32, length: u32, sustain: u32, looped: bool, rm: &mut ResourceManager, rl: &mut RaylibHandle, thread: &RaylibThread) -> Self {
		Self {
			spritesheet: rm.request(path, rl, thread).unwrap(),
			id,
			length,
			sustain,
			looped,
			current_frame: 0u32,
			sustain_countdown: 1u32
		}
	}

	pub fn rewind(&mut self) { self.current_frame = 0; }

	pub fn draw(&mut self, pos: Vector2, size: Vector2, rotation: f32, rl: &mut RaylibMode2D::<RaylibDrawHandle>/*rl: &mut RaylibDrawHandle*/) {
		
		// Changing frame if sustain is up
		if self.sustain_countdown == 0 {
			self.current_frame += 1;
			if self.looped {
				self.current_frame = self.current_frame % self.length;
			} else if self.current_frame >= self.length {
				self.current_frame = self.length - 1;
			}

			self.sustain_countdown = self.sustain;
		} else {
			self.sustain_countdown -= 1;
		}
		
		// Drawing texture
		if let Resource::Texture(texture) = self.spritesheet.as_ref() {
			rl.draw_texture_pro(
				texture,
				Rectangle::new(
					Self::UNIT.x * self.current_frame as f32,
					Self::UNIT.y * self.id as f32,
					Self::UNIT.x,
					Self::UNIT.y
				),
				Rectangle::new(
					pos.x + size.x/2., pos.y + size.y/2.,
					size.x, size.y
				), 
				size / 2.,
				rotation,
				Color::WHITE
			);
		}
	}
}