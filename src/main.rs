use raylib::prelude::*;

struct Variable {
    id: usize,
    value: Vector2,
}

struct Construction {
    vars: Vec<Variable>,
}

fn main() {
    let (mut rl, thread) = raylib::init().size(640, 480).title("Hello, world!").build();

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::new(64, 64, 64, 255));
        d.draw_text("Hello, world!", 12, 12, 20, Color::WHITE);
    }
}
