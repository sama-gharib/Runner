use raylib::prelude::*;

pub mod button;
pub mod application;

#[derive(Debug, Copy, Clone)]
pub enum Notification {
	MousePress (f32, f32),
	MouseRelease (f32, f32),
	KeyPress (KeyboardKey),
	MouseMove (f32, f32)
}

pub trait Model {
	fn notify(&mut self, n: Notification);
}