//! Resource I/O

use macroquad::prelude::*;
use macroquad::audio::*;

use std::rc::Rc;
use std::cell::RefCell;

use std::collections::HashMap;

#[derive(Debug)]
pub enum Resource {
	Texture (Texture2D),
	Sound {sound: Sound, volume: Rc::<RefCell::<f32>>}
}

impl Resource {
	pub fn play_if_sound(&self, looped: bool) {
		if let Self::Sound {sound, volume} = self {
			play_sound(
				sound,
				PlaySoundParams {
					volume: volume.as_ref().borrow().clone(),
					looped,
					..Default::default()
				}
			)
		}
	}
}

#[derive(Debug)]
pub enum ResourceError {
	UnknownExtension (String),
	LoadingError (String)
}

pub struct ResourceManager {
	resources: HashMap::<&'static str, Rc::<Resource>>,
	volume: Rc::<RefCell<f32>>
}

impl ResourceManager {
	pub fn new() -> Self {
		Self {
			resources: Default::default(),
			volume: Rc::new(RefCell::new(0.5))
		}
	}

	pub async fn request(&mut self, path: &'static str) -> Result<Rc::<Resource>, ResourceError> {

		if let Some(r) = self.resources.get(path) {
			Ok(Rc::clone(r))
		} else {

			let extension: String = path
				.chars()
				.rev()
				.take_while(|x| *x != '.')
				.collect::<String>()
				.chars()
				.rev()
				.map(|x| x.to_uppercase().collect::<Vec::<_>>())
				.flatten()
				.collect();

			match &extension[..] {
				"PNG" => match load_texture(path).await {
					Ok(t) => {
						t.set_filter(FilterMode::Nearest);
						self.resources.insert(path, Rc::new(Resource::Texture(t)));
						Ok(Rc::clone(&self.resources[path]))
					},
					Err(s) => {
						Err(ResourceError::LoadingError(s.to_string()))
					}
				},
				"WAV" => match load_sound(path).await {
					Ok(s) => {
						self.resources.insert(path, Rc::new(Resource::Sound{sound: s, volume: Rc::clone(&self.volume)}));
						Ok(Rc::clone(&self.resources[path]))
					},
					Err(e) => {
						Err(ResourceError::LoadingError(e.to_string()))
					}
				}
				_ => Err(ResourceError::UnknownExtension (extension))
			}
		}
	}

	pub fn set_volume(&mut self, v: f32) {
		let mut b = self.volume.borrow_mut();
		*b = v;
	}
}