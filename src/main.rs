use raylib::prelude::*;
use view::View;

mod util;
mod model;
mod view;

fn main() {
    let mut app = view::application::Application::from(
        model::application::Application::new()
    );

    let (mut rl, thread) = raylib::init()
        .size(800, 450)
        .title("Runner")
        .build();

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::BLACK);

        app.draw(&mut d);
    }

}
