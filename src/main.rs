use raylib::prelude::*;

pub mod builder;
// pub mod parser;

use builder::*;
// use parser::*;

fn main() {
    let (mut rl, thread) = raylib::init().size(640, 480).title("Hello, world!").build();

    let mut builder = Builder::new(vec![Step::Point]);

    builder.build();

    while !rl.window_should_close() {
        builder.update(&rl);

        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::new(64, 64, 64, 255));
        d.draw_text("Hello, world!", 12, 12, 20, Color::WHITE);

        builder.draw(&mut d);
    }
}
