use std::fmt;

use raylib::prelude::*;

#[derive(Debug)]
enum Tool {
    Point,
    Line,
    Circle,
}

// preemptive lol
impl fmt::Display for Tool {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

enum Shape {
    Point(Vector2),
    Line(Vector2, Vector2),
    Circle(Vector2, f32),
}

fn main() {
    let (mut rl, thread) = raylib::init().size(640, 480).title("Hello, world!").build();

    let mut first_pos = None;

    let mut selected_tool = Tool::Point;
    let mut shapes = Vec::new();

    while !rl.window_should_close() {
        let mouse = rl.get_mouse_position();

        if rl.is_mouse_button_pressed(MouseButton::MOUSE_LEFT_BUTTON) {
            match selected_tool {
                Tool::Point => shapes.push(Shape::Point(mouse)),
                Tool::Line => match first_pos {
                    Some(start) => {
                        shapes.push(Shape::Line(start, mouse));
                        first_pos = None;
                    }
                    None => first_pos = Some(mouse),
                },
                Tool::Circle => match first_pos {
                    Some(start) => {
                        shapes.push(Shape::Circle(start, start.distance_to(mouse)));
                        first_pos = None;
                    }
                    None => first_pos = Some(mouse),
                },
            }
        }

        if let Some(key) = rl.get_key_pressed() {
            match key {
                KeyboardKey::KEY_ONE | KeyboardKey::KEY_KP_1 => selected_tool = Tool::Point,
                KeyboardKey::KEY_TWO | KeyboardKey::KEY_KP_2 => {
                    selected_tool = Tool::Line;
                    first_pos = None;
                }
                KeyboardKey::KEY_THREE | KeyboardKey::KEY_KP_3 => {
                    selected_tool = Tool::Circle;
                    first_pos = None;
                }
                _ => {}
            }
        }

        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::new(64, 64, 64, 255));
        d.draw_text(
            format!("selected: {}", selected_tool).as_str(),
            12,
            12,
            20,
            Color::WHITE,
        );

        for shape in &shapes {
            match shape {
                Shape::Point(pos) => d.draw_circle_v(pos, 3.0, Color::GOLD),
                Shape::Line(a, b) => d.draw_line_v(a, b, Color::GOLD),
                Shape::Circle(center, radius) => {
                    d.draw_circle_lines(center.x as i32, center.y as i32, *radius, Color::GOLD)
                }
            }
        }

        match selected_tool {
            Tool::Point => d.draw_circle_v(mouse, 3.0, Color::GOLD),
            Tool::Line => match first_pos {
                Some(pos) => d.draw_line_v(pos, mouse, Color::GOLD),
                None => d.draw_circle_v(mouse, 1.0, Color::GOLD),
            },
            Tool::Circle => match first_pos {
                Some(pos) => {
                    d.draw_circle_v(pos, 1.0, Color::GOLD);
                    d.draw_circle_lines(
                        pos.x as i32,
                        pos.y as i32,
                        pos.distance_to(mouse),
                        Color::GOLD,
                    )
                }
                None => d.draw_circle_v(mouse, 1.0, Color::GOLD),
            },
        }
    }
}
