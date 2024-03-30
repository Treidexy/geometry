use raylib::prelude::*;

type VarId = usize;

#[derive(Clone, Copy)]
pub enum Step {
    Point,
    LineSectPoints(VarId, VarId),
    CircAtSect(VarId, VarId),
    LineSectPerp(VarId, VarId), // pt, line
}

#[derive(Clone, Copy)]
pub enum Construct {
    Point(Point),
    Line(Line),
    Circle(Circle),
}

type Point = Vector2;

#[derive(Clone, Copy)]
pub struct Line {
    pub pos: Point,
    pub dir: Vector2,
}

#[derive(Clone, Copy)]
pub struct Circle {
    pub pos: Point,
    pub radius: f32,
}

pub struct Builder {
    pub steps: Vec<Step>,

    constructs: Vec<Construct>,
    selected_input: usize,
}

impl Default for Builder {
    fn default() -> Self {
        Self {
            steps: Default::default(),
            constructs: Default::default(),
            selected_input: Default::default(),
        }
    }
}

impl Builder {
    fn get(&self, id: VarId) -> Construct {
        self.constructs[id]
    }

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
    pub fn new(steps: Vec<Step>) -> Self {
        Self {
            steps,

            constructs: Vec::new(),
            selected_input: !0usize,
        }
    }

    pub fn update(&mut self, rl: &RaylibHandle) {
        let mouse = rl.get_mouse_position();

        if !rl.is_mouse_button_down(MouseButton::MOUSE_LEFT_BUTTON) {
            self.selected_input = !0usize;
        } else if self.selected_input != !0usize {
            // let sel = &self.inputs[self.selected_input];
        }

        // for (i, input) in self.inputs.iter_mut().enumerate() {
        //     if mouse.distance_to(input.pos) < 6.9 {
        //         if !rl.is_mouse_button_down(MouseButton::MOUSE_LEFT_BUTTON) {
        //             self.selected_input = i;
        //         }
        //         break;
        //     }
        // }

        self.build();
    }

    pub fn build(&mut self) {
        self.constructs.clear();

        for &step in &self.steps {
            match step {
                Step::Point => {
                    self.constructs
                        .push(Construct::Point(Vector2::new(200.0, 200.0)));
                }
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
    }
}
