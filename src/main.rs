use application::Application;

use std::io::Read;
use game::world::tokenizer::Tokenizer;
use game::world::interpretor::Interpretor;

mod application;
mod ui;
mod game;

fn main() {
	let mut application = Application::new();

	application.run();
}