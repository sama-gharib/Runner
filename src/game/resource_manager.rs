//! Resource I/O

use macroquad::prelude::*;

use std::rc::Rc;

use std::collections::HashMap;

#[derive(Debug)]
pub enum Resource {
	Texture (Texture2D),
	// TODO: Add resource types
}

#[derive(Debug)]
pub enum ResourceError {
	UnknownExtension (String),
	LoadingError (String)
}

pub struct ResourceManager {
	resources: HashMap::<&'static str, Rc::<Resource>>
}

impl ResourceManager {
	pub fn new() -> Self {
		Self {
			resources: Default::default()
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
				}
				_ => Err(ResourceError::UnknownExtension (extension))
			}
		}
	}
}