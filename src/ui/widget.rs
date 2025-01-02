use raylib::prelude::*;

pub trait Widget {
	fn get_position(&self) -> Vector2;
	fn get_size(&self) -> Vector2;
	fn is_activated(&mut self) -> bool;
	fn get_id(&self) -> String;
	fn update(&mut self, rl: &mut RaylibHandle);
	fn draw(&self, rl: &mut RaylibDrawHandle);
	fn get_roles(&self) -> Vec::<SpecialRole>;
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum SpecialRole {
	StateChanger,
	WindowDestroyer,
	LevelSelector
}

pub struct Button {
	position: Vector2,
	size: Vector2,
	title: String,
	activated: bool,
	hovered: bool,
	pressed: bool,
	roles: Vec::<SpecialRole>
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
		self.title.clone()
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
		rl.draw_text(&self.title.chars().take_while(|x| *x!='.').collect::<String>(), self.position.x as i32 + 10, self.position.y as i32, self.size.y as i32, Color::BLACK);
	}

	fn get_roles(&self) -> Vec::<SpecialRole> {
 		self.roles.clone()
 	}
}

impl Button {
	pub fn new(position: Vector2, size: Vector2) -> Self {
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

	pub fn contains(&self, v: Vector2) -> bool {
		   self.position.x < v.x && self.position.x + self.size.x > v.x
		&& self.position.y < v.y && self.position.y + self.size.y > v.y
	}
}