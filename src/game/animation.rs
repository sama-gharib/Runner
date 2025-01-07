//! Animation utilities

use std::rc::Rc;
use macroquad::prelude::*;

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
	const UNIT: Vec2 = Vec2 { x: 32., y: 32. };

	pub async fn new(path: &'static str, id: u32, length: u32, sustain: u32, looped: bool, rm: &mut ResourceManager) -> Self {
		Self {
			spritesheet: rm.request(path).await.unwrap(),
			id,
			length,
			sustain,
			looped,
			current_frame: 0u32,
			sustain_countdown: 1u32
		}
	}

	pub fn rewind(&mut self) { self.current_frame = 0; }

	pub fn update(&mut self) {
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
	}

	pub fn draw(&self, pos: Vec2, size: Vec2, rotation: f32) {
		
		// Drawing texture
		if let Resource::Texture(texture) = self.spritesheet.as_ref() {
			draw_texture_ex(
				texture,
				pos.x, pos.y,
				WHITE,
				DrawTextureParams {
					dest_size: Some(vec2(size.x, size.y)),
					source: Some(Rect {
						x: Self::UNIT.x * self.current_frame as f32,
						y: Self::UNIT.y * self.id as f32,
						w: Self::UNIT.x,
						h: Self::UNIT.y
					}),
					rotation,
					pivot: None,
					..Default::default()
				}
			)
		}
	}
}