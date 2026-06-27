use crate::{draw::Draw, turtle::Turtle};

pub struct UnknownErr {}

pub struct Parser {
    turtle: Turtle,

    dist: i32,
    angle: f32,
}

pub fn new(dist: i32, angle: f32) -> Parser {
    Parser {
        turtle: Turtle::default(),
        dist,
        angle,
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
                _ => return Err(UnknownErr {}),
            }
        }

        return Ok(());
    }
}
