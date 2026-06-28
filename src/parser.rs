use core::fmt;

use crate::{draw::Draw, turtle::Turtle};

#[derive(Debug)]
pub struct UnknownErr {
    wrong_char: char,
}

impl fmt::Display for UnknownErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Character {} is not recognised", self.wrong_char)
    }
}

pub struct Parser {
    turtle: Turtle,

    dist: i32,
    angle: f32,

    stack: Vec<(i32, i32, f32)>,
}

pub fn new(dist: i32, angle: f32) -> Parser {
    Parser {
        turtle: Turtle::default(),
        dist,
        angle,
        stack: vec![],
    }
}

impl Parser {
    pub fn execute<D: Draw>(&mut self, instructions: &str, draw: &mut D) -> Result<(), UnknownErr> {
        for char in instructions.chars() {
            match char {
                'X' => (),
                'F' => self.turtle.forward(self.dist, draw),
                '+' => self.turtle.rotate(self.angle),
                '-' => self.turtle.rotate(-self.angle),
                '[' => self.push_stack(),
                ']' => {
                    let (posx, posy, angle) = self.stack.pop().unwrap();
                    self.turtle.relocate(posx, posy, angle);
                }
                _ => return Err(UnknownErr { wrong_char: char }),
            }
        }

        return Ok(());
    }

    fn push_stack(&mut self) {
        self.stack.push(self.turtle.get_location());
    }
}
