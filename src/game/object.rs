//! Game objects, physics simulation and game graphics

use raylib::prelude::*;

#[derive(Debug, Clone)]
pub enum ObjectKind {
	Player,
	Wall,
	Spike
}
impl From<&str> for ObjectKind {
	fn from(s: &str) -> Self {
		match s {
			"Spike" => Self::Spike,
			"Player" => Self::Player,
			_ => Self::Wall
		}
	}
}

/// Represents any object in-game
#[derive(Debug, Clone)]
pub struct Object {
	pub position: Vector2,
	pub size: Vector2,
	speed: Vector2,

	pub kind: ObjectKind,
	is_on_ground: bool,
	alive: bool,

	rotation: f32,

	trail: Vec::<Vector2>
}

impl From<ObjectKind> for Object {
	fn from(k: ObjectKind) -> Self {
		Self::new().kind(k)
	}	
}

impl Object {
	const TRAIL_LENGTH: usize = 30;

	/// Default constructor
	/// # Example
	/// ```
	/// let o = Object::new()
	///		.position(Vector2::new(100., 100.))
	///		.size(Vector2::new(50., 50.))
	///		.kind(ObjectKind::Wall);
	/// ```
	pub fn new() -> Self {
		Self {
			position: Vector2::zero(),
			size: Vector2::one() * 30.,
			speed: Vector2::zero(),
			kind: ObjectKind::Wall,
			is_on_ground: false,
			alive: true,
			rotation: 0.,
			trail: Default::default()
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

	/// Update function, has to be called once per game-loop
	/// Abstracts physics but not collisions, see Object::collide().
	pub fn update(&mut self, rl: &mut RaylibHandle) {
		if !self.alive { return }

		self.position += self.speed;

		// Trail management
		if self.speed.x != 0. || self.speed.y != 0. {
			self.trail.push(self.position + self.size * 0.5);
			if self.trail.len() > Self::TRAIL_LENGTH {
				self.trail.remove(0);
			}
		}

		// Inputs
		if let ObjectKind::Player = self.kind {
			self.speed.y += 1.;
			if rl.is_key_down(KeyboardKey::KEY_SPACE) && self.is_on_ground {
				let f = self.position + Vector2::new(self.size.x * 2., -self.size.y);
				let i = self.position;
				self.speed.y = self.speed.x * (f.y-i.y)/(f.x-i.x)-(f.x-i.x)/(2.*self.speed.x)-1./2.;
			}
		}

		// Flight/Landing related code
		if !self.is_on_ground {
			self.rotation += 4.;
		} else {
			self.rotation = 0.;
		}
		
		self.is_on_ground = false;
	}

	pub fn collide(&mut self, other: &Object) {
		// Players are the only object to "collide" other objects.
		// TODO: Switch to a match statement when adding other
		// colliding objects.
		if let ObjectKind::Player = self.kind {
			
			// Predicting future collision
			let future = self.position + self.speed;

			if future.x < other.position.x + other.size.x
			&& future.x + self.size.x > other.position.x
			&& future.y < other.position.y + other.size.y
			&& future.y + self.size.y > other.position.y {
				match other.kind {
					ObjectKind::Wall => if self.position.y + self.size.y <= other.position.y {
							self.speed.y *= 0.;
							// Snapping position to top of object
							self.position.y = other.position.y - self.size.y;

							self.is_on_ground = true;
						} else {
							self.alive = false;
						},
					ObjectKind::Spike => {
						// Tests collision more accurately (Spikes are triangles, not squares)
						let f = self.clone().position(future);
						if f.contains(other.position + Vector2::new(other.size.x/2., 0.))
						|| f.contains(other.position + Vector2::new(0., other.size.y))
						|| f.contains(other.position + other.size) {
							self.alive = false;
						}
					},
					ObjectKind::Player => todo!()
				}
			
			}
		}
	}

	pub fn draw(&self, rl: &mut RaylibDrawHandle, camera: &Camera2D) {
		
		let mut rl = rl.begin_mode2D(camera);
		
		// Trail related code
		for (i, w) in self.trail.windows(2).enumerate() {
			let trail_factor = i as f32 / Self::TRAIL_LENGTH as f32;
			let shade = (trail_factor * 255.) as u8;
			rl.draw_line_ex(w[0], w[1], trail_factor * self.size.y * 0.3, Color::new(255, 255, 255, shade));
		}
 
 		// Drawing the object itself
 		// TODO: Use sprites
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
					if self.alive { Color::WHITE } else { Color::RED }
				);
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

	fn contains(&self, v: Vector2) -> bool {
		   self.position.x <= v.x
		&& self.position.x + self.size.x >= v.x
		&& self.position.y <= v.y
		&& self.position.y + self.size.y >= v.y
	}
}