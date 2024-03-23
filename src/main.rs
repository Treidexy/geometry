use raylib::prelude::*;

type VarId = usize;

#[derive(Clone, Copy)]
enum Step {
    LineSectPoints(VarId, VarId),
    CircAtSect(VarId, VarId),
    LineSectPerp(VarId, VarId), // pt, line
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
    selected_input: usize,
}

impl Builder {
    fn get_point(&self, id: VarId) -> Point {
        match self.constructs[id] {
            Construct::Point(c) => c,
            _ => panic!(),
        }
    }

    fn get_line(&self, id: VarId) -> Line {
        match self.constructs[id] {
            Construct::Line(c) => c,
            _ => panic!(),
        }
    }

    fn get_circle(&self, id: VarId) -> Circle {
        match self.constructs[id] {
            Construct::Circle(c) => c,
            _ => panic!(),
        }
    }
}

impl Builder {
    pub fn update(&mut self, rl: &RaylibHandle) {
        let mouse = rl.get_mouse_position();

        if !rl.is_mouse_button_down(MouseButton::MOUSE_LEFT_BUTTON) {
            self.selected_input = !0usize;
        } else if self.selected_input != !0usize {
            self.inputs[self.selected_input] = mouse;
        }

        for (i, input) in self.inputs.iter_mut().enumerate() {
            if mouse.distance_to(*input) < 6.9 {
                if !rl.is_mouse_button_down(MouseButton::MOUSE_LEFT_BUTTON) {
                    self.selected_input = i;
                }
                break;
            }
        }

        self.build();
    }

    pub fn build(&mut self) {
        self.constructs.clear();

        for &input in &self.inputs {
            self.constructs.push(Construct::Point(input));
        }

        for &step in &self.steps {
            match step {
                Step::LineSectPoints(a, b) => {
                    self.constructs.push(Construct::Line(Line {
                        pos: self.get_point(a),
                        dir: (self.get_point(b) - self.get_point(a)).normalized(),
                    }));
                }
                Step::CircAtSect(at, sect) => {
                    self.constructs.push(Construct::Circle(Circle {
                        pos: self.get_point(at),
                        radius: self.get_point(at).distance_to(self.get_point(sect)),
                    }));
                }
                Step::LineSectPerp(at, perp) => self.constructs.push(Construct::Line(Line {
                    pos: self.get_point(at),
                    dir: {
                        let d = self.get_line(perp).dir;
                        Vector2::new(-d.y, d.x)
                    },
                })),
            }
        }
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle) {
        for &construct in &self.constructs {
            match construct {
                Construct::Point(pos) => {
                    d.draw_circle_v(pos, 3.0, Color::GOLD);
                }
                Construct::Line(Line { pos, dir }) => {
                    d.draw_line_v(
                        pos - dir.scale_by(1000.0),
                        pos + dir.scale_by(1000.0),
                        Color::GOLD,
                    );
                }
                Construct::Circle(Circle { pos, radius }) => {
                    d.draw_circle_lines(pos.x as i32, pos.y as i32, radius, Color::GOLD);
                }
            }
        }

        for (i, &input) in self.inputs.iter().enumerate() {
            d.draw_circle_v(
                input,
                if i == self.selected_input { 7.0 } else { 5.0 },
                Color::RED,
            );
        }
    }
}

fn main() {
    let (mut rl, thread) = raylib::init().size(640, 480).title("Hello, world!").build();

    let mut builder = Builder {
        inputs: vec![
            Vector2::new(50.0, 100.0),
            Vector2::new(300.0, 300.0),
            Vector2::new(300.0, 300.0),
        ],
        steps: vec![
            Step::LineSectPoints(0, 1),
            Step::CircAtSect(0, 2),
            Step::LineSectPerp(1, 3),
        ],
        constructs: Vec::new(),
        selected_input: !0usize,
    };

    builder.build();

    while !rl.window_should_close() {
        builder.update(&rl);

        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::new(64, 64, 64, 255));
        d.draw_text("Hello, world!", 12, 12, 20, Color::WHITE);

        builder.draw(&mut d);
    }
}
