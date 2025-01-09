//! Group of widget abstraction

use macroquad::prelude::*;
use super::widget::Widget;
use super::widget::SpecialRole;

/// Represents a "tab" in the UI
pub struct Menu {
	id: String,
	widgets: Vec::<Box::<dyn Widget>>
}

impl Menu {
	pub fn new(id: &str) -> Self {
		Self {
			id: String::from(id),
			widgets: Vec::<Box::<dyn Widget>>::new()
		}
	}

	pub fn add_widget(mut self, w: Box::<dyn Widget>) -> Self {
		self.widgets.push(w);
		self
	}

	/// Broadcasts the draw wall to every widget in menu
	pub fn draw(&self) {
		for widget in self.widgets.iter() {
			widget.draw();
		}
	}

	/// Broadcasts the update call to every widget in menu
	pub fn update(&mut self) {
		for widget in self.widgets.iter_mut() {
			widget.update();
		}
	}

	/// Collects ids and role from activated widgets
	pub fn activations(&mut self) -> Vec::<(String, Vec::<SpecialRole>, f32)> {
		self.widgets
			.iter_mut()
			.filter_map(|x| {
				let f = x.activation_factor();
				if f != 0. {
					Some(
						(x.get_id(), x.get_roles(), f)
					)
				} else {
					None
				}
			})
			.collect()
	}

	pub fn id(&self) -> String {
		self.id.clone()
	} 
}