//! Group of widget abstraction

use raylib::prelude::*;
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
	pub fn draw(&self, rl: &mut RaylibDrawHandle) {
		for widget in self.widgets.iter() {
			widget.draw(rl);
		}
	}

	/// Broadcasts the update call to every widget in menu
	pub fn update(&mut self,  rl: &mut RaylibHandle) {
		for widget in self.widgets.iter_mut() {
			widget.update(rl);
		}
	}

	/// Collects ids and role from activated widgets
	pub fn activations(&mut self) -> Vec::<(String, Vec::<SpecialRole>)> {
		self.widgets
			.iter_mut()
			.filter_map(|x| if x.is_activated() { Some((x.get_id(), x.get_roles())) } else { None })
			.collect()
	}

	pub fn id(&self) -> String {
		self.id.clone()
	} 
}