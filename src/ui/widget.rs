use raylib::prelude::*;

pub trait Widget {
	fn get_position(&self) -> Vector2;
	fn get_size(&self) -> Vector2;
	fn is_activated(&mut self) -> bool;
	fn get_id(&self) -> String;
	fn update(&mut self, rl: &mut RaylibHandle);
	fn draw(&self, rl: &mut RaylibDrawHandle);
}

pub struct Button {
	position: Vector2,
	size: Vector2,
	title: String,
	menu_changer: bool,
	activated: bool
}

impl Widget for Button {
	fn get_position(&self) -> Vector2 {
		self.position
	}
	fn get_size(&self) -> Vector2 {
		self.size
	}
	fn is_activated(&mut self) -> bool {
		std::mem::replace(&mut self.activated, false)
	}
	fn get_id(&self) -> String {
		format!("{}{}", if self.menu_changer { crate::ui::Ui::CHANGE_STATE_COMMAND } else { "" }, self.title)
	}
	fn update(&mut self, rl: &mut RaylibHandle) {
		let mouse = rl.get_mouse_position();

		self.activated = self.contains(mouse) && rl.is_mouse_button_released(MouseButton::MOUSE_BUTTON_LEFT);
	}

	fn draw(&self, rl: &mut RaylibDrawHandle) {
		rl.draw_rectangle_v(self.position, self.size, Color::WHITE);
		rl.draw_text(&self.title, self.position.x as i32 + 10, self.position.y as i32, self.size.y as i32, Color::BLACK);
	}
}

impl Button {
	pub fn new(position: Vector2, size: Vector2) -> Self {
		Self {
			position,
			size,
			title: String::new(),
			menu_changer: false,
			activated: false
		}
	}

	pub fn title(mut self, x: &str) -> Self {
		self.title = x.to_string();
		self
	}

	pub fn state_changer(mut self) -> Self {
		self.menu_changer = true;
		self
	}

	pub fn contains(&self, v: Vector2) -> bool {
		   self.position.x < v.x && self.position.x + self.size.x > v.x
		&& self.position.y < v.y && self.position.y + self.size.y > v.y
	}
}