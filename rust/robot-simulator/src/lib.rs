// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.
use Direction::*;

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

pub struct Robot {
    x: isize,
    y: isize,
    direction: Direction,
}

impl Robot {
    pub fn new(x: isize, y: isize, d: Direction) -> Self {
        Robot {
            x: x,
            y: y,
            direction: d,
        }
    }

    pub fn turn_right(self) -> Self {
        let new_direction = match self.direction {
            North => East,
            East => South,
            South => West,
            West => North,
        };
        Robot::new(self.x, self.y, new_direction)
    }

    pub fn turn_left(self) -> Self {
        let new_direction = match self.direction {
            North => West,
            East => North,
            South => East,
            West => South,
        };
        Robot::new(self.x, self.y, new_direction)
    }

    pub fn advance(self) -> Self {
        let (new_x, new_y) = match self.direction {
            North => (self.x, self.y + 1),
            East => (self.x + 1, self.y),
            South => (self.x, self.y - 1),
            West => (self.x - 1, self.y),
        };
        Robot::new(new_x, new_y, self.direction)
    }

    pub fn instructions(self, instructions: &str) -> Self {
        instructions.chars().fold(self, |accu, c| {
            match c {
                'A' => accu.advance(),
                'L' => accu.turn_left(),
                'R' => accu.turn_right(),
                _ => unreachable!(),
            }
        })
    }

    pub fn position(&self) -> (isize, isize) {
        (self.x, self.y)
    }

    pub fn direction(&self) -> &Direction {
        &self.direction
    }
}
