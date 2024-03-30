use std::str::Chars;

use raylib::math::Vector2;

use crate::builder::{self, *};

struct Parser {
    src: Vec<u8>,
    pos: usize,
    builder: Builder,

    names: Vec<String>,
}

impl Parser {
    pub fn parse(src: &str) -> Builder {
        let mut parser = Parser {
            src: src.as_bytes().to_vec(),
            pos: 0,
            builder: Builder::new(Vec::new(), Vec::new()),
        };

        while parser.pos < parser.src.len() {
            parser.parse_stmt();
        }
        parser.builder
    }

    fn parse_stmt(&mut self) {
        let cmd = self.eat_word();
        match cmd.as_str() {
            "given" => {
                while self.peek() != '.' {
                    let typo = self.eat_word();
                    if typo != "point" {
                        panic!("yanis, thanusn't a valid Given type");
                    }

                    let name = self.eat_word();

                    self.names.push(name);
                    self.builder.inputs.push(Input {
                        pos: Vector2::zero(),
                        kind: InputKind::Free,
                    });

                    if self.peek() != ',' {
                        panic!();
                    }

                    self.eat_char();
                }
            }
            "let" => {}
            _ => {}
        }
    }

    fn peek(&mut self) -> char {
        self.eat_space();

        self.src[self.pos] as char
    }

    fn eat_space(&mut self) {
        while self.src[self.pos].is_ascii_whitespace() {
            self.pos += 1;
        }
    }

    fn eat_char(&mut self) -> char {
        self.eat_space();
        let c = self.src[self.pos];
        self.pos += 1;
        c as char
    }

    fn eat_word(&mut self) -> String {
        self.eat_space();

        let mut word = String::new();
        if !self.src[self.pos].is_ascii_alphabetic() {
            return word;
        }

        while self.src[self.pos].is_ascii_alphanumeric() {
            word.push(self.src[self.pos] as char);
            self.pos += 1;
        }

        word
    }
}
