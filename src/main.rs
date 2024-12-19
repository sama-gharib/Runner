use raylib::prelude::*;

mod util;
mod model;

use crate::model::application::Application;

fn main() {
    let mut app = Application::new();

    app.run();
}
