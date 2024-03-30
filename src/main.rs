use raylib::prelude::*;

pub mod builder;
pub mod parser;

use builder::*;
use parser::*;

fn main() {
    let (mut rl, thread) = raylib::init().size(640, 480).title("Hello, world!").build();

    let mut builder = Builder::new(
        vec![
            Input::free(Vector2::new(50.0, 100.0)),
            Input::sect(Vector2::new(250.0, 200.0), 4),
            Input::free(Vector2::new(300.0, 300.0)),
        ],
        vec![
            Step::CircAtSect(0, 1),
            Step::LineSectPoints(0, 2),
            Step::LineSectPerp(2, 4),
        ],
    );

    builder.build();

    while !rl.window_should_close() {
        builder.update(&rl);

        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::new(64, 64, 64, 255));
        d.draw_text("Hello, world!", 12, 12, 20, Color::WHITE);

        builder.draw(&mut d);
    }
}
