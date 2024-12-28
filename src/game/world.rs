use raylib::prelude::*;

use super::object::*;

pub struct World {
	objects: Vec::<Object>,
	camera: Camera2D
}

impl Default for World {
	fn default() -> Self {
		let mut r = Self::new();
		
		r.objects.push(Object::new().kind(ObjectKind::Player).speed(Vector2::new(5., 0.)));
		r.objects.push(Object::new().kind(ObjectKind::Spike).position(Vector2::one() * 200.));
		r.objects.push(Object::new().kind(ObjectKind::Wall).position(Vector2::new(0., 230.)).size(Vector2::new(1000., 300.)));
		r.objects.push(Object::new().kind(ObjectKind::Wall).position(Vector2::new(970., 100.)).size(Vector2::new(30., 200.)));

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