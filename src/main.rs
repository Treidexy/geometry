use raylib::prelude::*;

type VarId = usize;

enum Step {
    LineSectPoints(VarId, VarId),
}

#[derive(Clone, Copy)]
enum Construct {
    Point(Point),
    Line(Line),
    Circle(Circle),
}

type Point = Vector2;

#[derive(Clone, Copy)]
struct Line {
    pos: Point,
    dir: Vector2,
}

#[derive(Clone, Copy)]
struct Circle {
    pos: Point,
    radius: f32,
}

struct Builder {
    inputs: Vec<Vector2>,
    steps: Vec<Step>,

    constructs: Vec<Construct>,
}

impl Builder {
    pub fn build(&mut self) {
        for step in &self.steps {
            match step {
                Step::LineSectPoints(a, b) => {
                    self.constructs.push(Construct::Line(Line { pos: self.inputs[*a], dir: (self.inputs[*b] - self.inputs[*a]).normalized(),}));
                }
            }
        }
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle) {
        for &construct in &self.constructs {
            match construct {
                Construct::Point(pos) => {
                    d.draw_circle_v(pos, 3.0, Color::GOLD);
                }
                Construct::Line(Line{ pos, dir}) => {
                    d.draw_line_v(pos - dir.scale_by(1000.0), pos + dir.scale_by(1000.0), Color::GOLD);
                }
                Construct::Circle(Circle {pos, radius}) => {
                    d.draw_circle_lines(pos.x as i32, pos.y as i32, radius, Color::GOLD);
                }
            }
        }

        for &input in &self.inputs {
            d.draw_circle_v(input, 5.0, Color::RED);
        }
    }
}

fn main() {
    let (mut rl, thread) = raylib::init().size(640, 480).title("Hello, world!").build();

    let mut builder = Builder {
        inputs: vec![Vector2::new(3.0,3.0),Vector2::new(300.0,300.0)],
        steps: vec![Step::LineSectPoints(0, 1)],
        constructs: Vec::new(),
    };

    builder.build();

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::new(64, 64, 64, 255));
        d.draw_text("Hello, world!", 12, 12, 20, Color::WHITE);

        builder.draw(&mut d);
    }
}
