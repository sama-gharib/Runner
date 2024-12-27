use crate::model::Model;

use raylib::prelude::*;

pub mod application;
pub mod button;

pub trait View {
	fn draw(&mut self, handle: &mut RaylibDrawHandle);
	fn get_model(&mut self) -> &mut dyn Model;
}