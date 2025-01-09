//! Level semantic and camera management

use std::fs::File;
use std::io::Read;

use macroquad::prelude::*;

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
	camera: Camera2D,
	playing: bool
}


impl World {

	/// Loads level from file
	/// TODO: Handle errors more cleanly
	pub async fn from(arg: (&str, &mut ResourceManager)) -> Self {
		let mut s = String::new();
		File::open(&format!("res/levels/{}", arg.0)).unwrap().read_to_string(&mut s).unwrap();
		Interpretor::interpret(
			Tokenizer::tokenize(&s, arg.1).await.unwrap()
		).unwrap()
	}

	/// Default empty world constructor
	pub fn new() -> Self {
		Self {
			objects: Vec::<Object>::new(),
			camera: Camera2D {
				zoom: vec2(1./400., 1./225.),
				..Default::default()
			},
			playing: true
		}
	}

	pub fn is_playing(&self) -> bool { self.playing } 

	/// Broadcasts the update call on every object of world, handle collisions
	/// and move camera.
	/// Has to be called once per game loop
	pub fn update(&mut self) {

		self.playing = false;

		for i in 0..self.objects.len() {			
			// Collision code
			let splitted = self.objects.split_at_mut(i+1);
			for o in splitted.1 {
				splitted.0.last_mut().unwrap().collide(o);
			}
			
			self.objects[i].update();
			
			// Camera movement
			if let ObjectKind::Player {..} = self.objects[i].kind {
				self.camera.target = self.camera.target + (
					self.objects[i].position + self.objects[i].size/2. + vec2(screen_width() * 2./7., 0.) -
					self.camera.target
				) * 0.1;

				if self.objects[i].is_alive() { self.playing = true; }
			}
		}
	}

	/// Broadcasts the draw call on every object in world
	pub fn draw(&mut self) {

		set_camera(&self.camera);

		for o in self.objects.iter_mut() {
			o.draw();
		}
		set_default_camera();
	}

	pub fn add_object(&mut self, o: Object) {
		self.objects.push(o);
	}
}