use raylib::prelude::*;
use super::widget::Widget;

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

	pub fn draw(&self, rl: &mut RaylibDrawHandle) {
		for widget in self.widgets.iter() {
			widget.draw(rl);
		}
	}

	pub fn update(&mut self,  rl: &mut RaylibHandle) {
		for widget in self.widgets.iter_mut() {
			widget.update(rl);
		}
	}

	pub fn activations(&mut self) -> Vec::<String> {
		self.widgets
			.iter_mut()
			.filter_map(|x| if x.is_activated() { Some(x.get_id()) } else { None })
			.collect()
	}

	pub fn id(&self) -> String {
		self.id.clone()
	} 
}