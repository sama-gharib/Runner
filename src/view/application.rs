use crate::model::Notification;
use super::{View, button::*};
use crate::model::Model;
use crate::model::application::ApplicationState;

use raylib::prelude::*;

pub struct Application {

	main_menu: Vec::<Button>,

	last_mouse_pos: Vector2,

	model: crate::model::application::Application
}

impl View for Application {
	fn draw(&mut self, handle: &mut RaylibDrawHandle) {
		
		let mouse = handle.get_mouse_position();
		
		let notification = if handle.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
			Some(Notification::MousePress(mouse.x, mouse.y))
		} else if handle.is_mouse_button_released(MouseButton::MOUSE_BUTTON_LEFT) {
			Some(Notification::MouseRelease(mouse.x, mouse.y))
		} else if mouse != self.last_mouse_pos {
			self.last_mouse_pos = mouse;
			Some(Notification::MouseMove(mouse.x, mouse.y))
		} else {
			None
		};

		if let Some(notification) = notification {
			for b in self.main_menu.iter_mut() {
				b.get_model().notify(notification);
			}
		}

		match self.model.state() {
		    ApplicationState::MainMenu => {
		    	for b in self.main_menu.iter_mut() {
		    		b.draw(handle);
		    	}
		    }
		    | ApplicationState::Game | ApplicationState::ConfirmQuit => todo!(),
		}
	}

	fn get_model(&mut self) -> &mut dyn Model {
		&mut self.model
	}
}

impl From<crate::model::application::Application> for Application {
	fn from(model: crate::model::application::Application) -> Self {
		let mut r = Self {
			main_menu: Vec::<Button>::new(),
			last_mouse_pos: Vector2::new(0.0, 0.0),
			model
		};

		for b in r.model.main_menu() {
			r.main_menu.push(super::button::Button::from(b));
		}

		r
	}
}