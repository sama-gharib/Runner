//! Game objects, physics simulation and game graphics

use macroquad::prelude::*;

use super::animation::*;
use super::resource_manager::*;

#[derive(Debug, Clone)]
pub enum PlayerState {
	Running,
	Jumping,
	Dying
}

#[derive(Debug, Clone)]
pub enum ObjectKind {
	Player {state: PlayerState, run: Animation, jump: Animation, die: Animation},
	Wall,
	Spike
}

impl ObjectKind {
	pub async fn from(arg: (&str, &mut ResourceManager)) -> Self {
		match arg.0 {
			"Spike" => Self::Spike,
			"Player" => Self::player(arg.1).await,
			_ => Self::Wall
		}
	}

	async fn player(rm: &mut ResourceManager) -> Self {
		Self::Player {
			state: PlayerState::Jumping,
			run: Animation::new("res/sprites/player.png", 0, 6, 3, true, rm).await,
			jump: Animation::new("res/sprites/player.png", 1, 3, 6, false, rm).await,
			die: Animation::new("res/sprites/player.png", 2, 5, 6, false, rm).await
		}
	}
}


/// Represents any object in-game
#[derive(Debug, Clone)]
pub struct Object {
	pub position: Vec2,
	pub size: Vec2,
	speed: Vec2,

	pub kind: ObjectKind,
	is_on_ground: bool,
	alive: bool,

	rotation: f32,

	trail: Vec::<Vec2>
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
	///		.position(Vec2::new(100., 100.))
	///		.size(Vec2::new(50., 50.))
	///		.kind(ObjectKind::Wall);
	/// ```
	pub fn new() -> Self {
		Self {
			position: Vec2::ZERO,
			size: Vec2::ONE * 30.,
			speed: Vec2::ZERO,
			kind: ObjectKind::Wall,
			is_on_ground: false,
			alive: true,
			rotation: 0.,
			trail: Default::default()
		}
	}

	pub fn position(mut self, x: Vec2) -> Self {
		self.position = x;
		self
	}

	pub fn size(mut self, x: Vec2) -> Self {
		self.size = x;
		self
	}

	pub fn speed(mut self, x: Vec2) -> Self {
		self.speed = x;
		self
	}

	pub fn kind(mut self, x: ObjectKind) -> Self {
		self.kind = x;
		self
	}

	/// Update function, has to be called once per game-loop
	/// Abstracts physics but not collisions, see Object::collide().
	pub fn update(&mut self) {
		// Trail management
		self.trail.push(self.position + self.size * 0.5);
		if self.trail.len() > Self::TRAIL_LENGTH {
			self.trail.remove(0);
		}

		self.position += self.speed;

		if !self.alive {
			if let ObjectKind::Player {state, ..} = &mut self.kind {
				self.speed.y += 1.;
				self.speed.x *= 0.95;
				*state = PlayerState::Dying;
			}
		} else {

			// Inputs
			if let ObjectKind::Player {state, jump, ..} = &mut self.kind {
				self.speed.y += 1.;
				if is_key_down(KeyCode::Space) && self.is_on_ground {
					let f = self.position + Vec2::new(self.size.x * 2., -self.size.y);
					let i = self.position;
					self.speed.y = self.speed.x * (f.y-i.y)/(f.x-i.x)-(f.x-i.x)/(2.*self.speed.x)-1./2.;
				}
				if self.is_on_ground {
					*state = PlayerState::Running;
					jump.rewind();
				} else {
					*state = PlayerState::Jumping;
				}
			}

			// Flight/Landing related code
			if !self.is_on_ground {
				self.rotation += 0.2;
			} else {
				self.rotation = 0.;
			}
			
			self.is_on_ground = false;
		}
	}

	pub fn collide(&mut self, other: &Object) {
		// Players are the only object to "collide" other objects.
		// TODO: Switch to a match statement when adding other
		// colliding objects.
		if let ObjectKind::Player {..} = self.kind {
			
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
							if self.position.x + self.size.x <= other.position.x {
								self.speed.x = 0.;
								self.position.x = other.position.x - self.size.x;
							} else if self.position.y >= other.position.y + other.size.y {
								self.speed.y = 0.;
								self.position.y = other.position.y + other.size.y;
							}
							
							self.alive = false;
						},
					ObjectKind::Spike => {
						// Tests collision more accurately (Spikes are triangles, not squares)
						let f = self.clone().position(future);
						if f.contains(other.position + Vec2::new(other.size.x/2., 0.))
						|| f.contains(other.position + Vec2::new(0., other.size.y))
						|| f.contains(other.position + other.size) {
							self.speed.x = 0.;
							self.speed.y = 0.;
							self.alive = false;
						}
					},
					ObjectKind::Player {..} => todo!()
				}
			
			}
		}
	}

	pub fn draw(&mut self) {
				
		// Trail related code
		for (i, w) in self.trail.windows(2).enumerate() {
			let trail_factor = i as f32 / Self::TRAIL_LENGTH as f32;
			draw_line(
				w[0].x, w[0].y,
				w[1].x, w[1].y,
				trail_factor * self.size.y * 0.3,
				Color::new(1., 1., 1., trail_factor));
		}
 
 		// Drawing the object itself
 		// TODO: Use more sprites
		match &mut self.kind {
			ObjectKind::Player {state, run, jump, die} => {
				/*rl.draw_rectangle_pro(
					Rectangle::new(
						self.position.x + self.size.x/2.,
						self.position.y + self.size.y/2.,
						self.size.x,
						self.size.y
					),
					self.size / 2.,
					self.rotation,
					if self.alive { Color::WHITE } else { Color::RED }
				);*/
				match state {
					PlayerState::Running => run.draw(self.position, self.size, self.rotation),
					PlayerState::Jumping => jump.draw(self.position, self.size, self.rotation),
					PlayerState::Dying => die.draw(self.position, self.size, 0.)
				}
			},
			ObjectKind::Wall => {
				draw_rectangle(
					self.position.x,
					self.position.y,
					self.size.x,
					self.size.y,
					WHITE);
			},
			ObjectKind::Spike => {
				draw_triangle(
					self.position + Vec2::new(self.size.x/2., 0.0),
					self.position + Vec2::new(0., self.size.y),
					self.position + self.size,
					WHITE
				);
			}
		}
	}

	fn contains(&self, v: Vec2) -> bool {
		   self.position.x <= v.x
		&& self.position.x + self.size.x >= v.x
		&& self.position.y <= v.y
		&& self.position.y + self.size.y >= v.y
	}
}