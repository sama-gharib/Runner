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
	activated: bool,
	hovered: bool,
	pressed: bool,
	window_destroyer: bool
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
		format!(
			"{}{}",
		
			if self.menu_changer {
				crate::ui::Ui::CHANGE_STATE_COMMAND
			} else if self.window_destroyer {
				crate::ui::Ui::DESTROY_WINDOW_COMMAND
			} else { "" },
		
			self.title
		)
	}
	fn update(&mut self, rl: &mut RaylibHandle) {
		let mouse = rl.get_mouse_position();

		let p = rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT);
		let r = rl.is_mouse_button_released(MouseButton::MOUSE_BUTTON_LEFT);

		self.hovered = self.contains(mouse);
		self.activated = self.pressed && self.hovered && r;
		self.pressed = self.hovered && p || (self.pressed && !self.activated && !r);
	}

	fn draw(&self, rl: &mut RaylibDrawHandle) {
		rl.draw_rectangle_v(self.position, self.size, if self.pressed { Color::new(200, 200, 200, 255) } else if self.hovered { Color::GRAY } else { Color::WHITE });
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
			activated: false,
			hovered: false,
			pressed: false,
			window_destroyer: false
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

	pub fn window_destroyer(mut self) -> Self {
		self.window_destroyer = true;
		self
	}

	pub fn contains(&self, v: Vector2) -> bool {
		   self.position.x < v.x && self.position.x + self.size.x > v.x
		&& self.position.y < v.y && self.position.y + self.size.y > v.y
	}
}