use raylib::prelude::*;

pub struct World {
	objects: Vec::<Object>,
	camera: Camera2D
}

impl Default for World {
	fn default() -> Self {
		let mut r = Self::new();
		
		r.objects.push(Object::new().kind(ObjectKind::Player).speed(Vector2::new(5., 0.)));
		r.objects.push(Object::new().kind(ObjectKind::Spike).position(Vector2::one() * 200.));
		r.objects.push(Object::new().kind(ObjectKind::Wall).position(Vector2::new(0., 230.)).size(Vector2::new(10000., 300.)));

		r
	}
}

impl World {
	pub fn new() -> Self {
		Self {
			objects: Vec::<Object>::new(),
			camera: Camera2D {zoom: 1., offset: Vector2::new(200., 225.), ..Default::default()}
		}
	}

	pub fn update(&mut self, rl: &mut RaylibHandle) {
		for i in 0..self.objects.len() {			
			let splitted = self.objects.split_at_mut(i+1);
			for o in splitted.1 {
				splitted.0.last_mut().unwrap().collide(o);
			}
			
			self.objects[i].update(rl);
			if let ObjectKind::Player = self.objects[i].kind {
				self.camera.target = self.camera.target + (self.objects[i].position + self.objects[i].size/2. - self.camera.target) * 0.25;
			}
		}
	}

	pub fn draw(&self, rl: &mut RaylibDrawHandle) {

		for o in self.objects.iter() {
			o.draw(rl, &self.camera);
		}
	}
}

pub enum ObjectKind {
	Player,
	Wall,
	Spike
}

pub struct Object {
	position: Vector2,
	size: Vector2,
	speed: Vector2,

	kind: ObjectKind,
	is_on_ground: bool,
	alive: bool,

	rotation: f32
}

impl Object {
	pub fn new() -> Self {
		Self {
			position: Vector2::zero(),
			size: Vector2::one() * 30.,
			speed: Vector2::zero(),
			kind: ObjectKind::Wall,
			is_on_ground: false,
			alive: true,
			rotation: 0.
		}
	}

	pub fn position(mut self, x: Vector2) -> Self {
		self.position = x;
		self
	}

	pub fn size(mut self, x: Vector2) -> Self {
		self.size = x;
		self
	}

	pub fn speed(mut self, x: Vector2) -> Self {
		self.speed = x;
		self
	}

	pub fn kind(mut self, x: ObjectKind) -> Self {
		self.kind = x;
		self
	}

	pub fn update(&mut self, rl: &mut RaylibHandle) {
		self.position += self.speed;

		if let ObjectKind::Player = self.kind {
			self.speed.y += 1.;
			if rl.is_key_down(KeyboardKey::KEY_SPACE) && self.is_on_ground {
				self.speed.y = -12.;
			}
		}
		if !self.is_on_ground {
			self.rotation += 4.;
		} else {
			self.rotation = 0.;
		}
		
		self.is_on_ground = false;
	}

	pub fn collide(&mut self, other: &Object) {
		if let ObjectKind::Player = self.kind {
			let future = self.position + self.speed;

			if future.x < other.position.x + other.size.x
			&& future.x + self.size.x > other.position.x
			&& future.y < other.position.y + other.size.y
			&& future.y + self.size.y > other.position.y {
				if self.position.y + self.size.y <= other.position.y {
					self.speed.y *= 0.;
					self.is_on_ground = true;
				}
			
			}
		}
	}

	pub fn draw(&self, rl: &mut RaylibDrawHandle, camera: &Camera2D) {
		
		let mut rl = rl.begin_mode2D(camera);
		match self.kind {
			ObjectKind::Player => {
				rl.draw_rectangle_pro(
					Rectangle::new(
						self.position.x + self.size.x/2.,
						self.position.y + self.size.y/2.,
						self.size.x,
						self.size.y
					),
					self.size / 2.,
					self.rotation,
					Color::WHITE
				);
				// rl.draw_rectangle_v(self.position, self.size, Color::WHITE);
			},
			ObjectKind::Wall => {
				rl.draw_rectangle_v(self.position, self.size, Color::WHITE);
			},
			ObjectKind::Spike => {
				rl.draw_triangle_fan(
					&[
						self.position + Vector2::new(self.size.x/2., 0.0),
						self.position + Vector2::new(0., self.size.y),
						self.position + self.size
					],
					Color::WHITE
				);
			}
		}
	}
}