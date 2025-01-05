//! Level semantic and camera management

use std::fs::File;
use std::io::Read;

use raylib::prelude::*;

use super::resource_manager::*;
use super::object::*;
use tokenizer::Tokenizer;
use interpretor::Interpretor;

pub mod tokenizer;
pub mod interpretor;


/// Abstracts a level
#[derive(Debug)]
pub struct World {
	objects: Vec::<Object>,
	camera: Camera2D
}

impl From<(&str, &mut ResourceManager, &mut RaylibHandle, &RaylibThread)> for World {
	
	/// Loads level from file
	/// TODO: Handle errors more cleanly
	fn from(arg: (&str, &mut ResourceManager, &mut RaylibHandle, &RaylibThread)) -> Self {
		let mut s = String::new();
		File::open(&format!("res/levels/{}", arg.0)).unwrap().read_to_string(&mut s).unwrap();
		Interpretor::interpret(
			Tokenizer::tokenize(&s, arg.1, arg.2, arg.3).unwrap()
		).unwrap()
	}
}

impl World {

	/// Default empty world constructor
	pub fn new() -> Self {
		Self {
			objects: Vec::<Object>::new(),
			camera: Camera2D {zoom: 1., offset: Vector2::new(200., 225.), ..Default::default()}
		}
	}

	/// Broadcasts the update call on every object of world, handle collisions
	/// and move camera.
	/// Has to be called once per game loop
	pub fn update(&mut self, rl: &mut RaylibHandle) {
		for i in 0..self.objects.len() {			
			// Collision code
			let splitted = self.objects.split_at_mut(i+1);
			for o in splitted.1 {
				splitted.0.last_mut().unwrap().collide(o);
			}
			
			self.objects[i].update(rl);
			
			// Camera movement
			if let ObjectKind::Player {..} = self.objects[i].kind {
				self.camera.target = self.camera.target + (self.objects[i].position + self.objects[i].size/2. - self.camera.target) * 0.25;
			}
		}
	}

	/// Broadcasts the draw call on every object in world
	pub fn draw(&mut self, rl: &mut RaylibDrawHandle) {

		for o in self.objects.iter_mut() {
			o.draw(rl, &self.camera);
		}
	}

	pub fn add_object(&mut self, o: Object) {
		self.objects.push(o);
	}
}