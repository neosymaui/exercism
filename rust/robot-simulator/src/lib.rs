// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(PartialEq, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

pub struct Robot {
    x: i32,
    y: i32,
    d: Direction,
}

impl Robot {
    pub fn new(x: i32, y: i32, d: Direction) -> Self {
        Robot { x, y, d }
    }

    pub fn turn_right(self) -> Self {
        let d = match self.d {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        };
        Robot {
            x: self.x,
            y: self.y,
            d,
        }
    }

    pub fn turn_left(self) -> Self {
        let d = match self.d {
            Direction::North => Direction::West,
            Direction::West => Direction::South,
            Direction::South => Direction::East,
            Direction::East => Direction::North,
        };
        Robot {
            x: self.x,
            y: self.y,
            d,
        }
    }

    pub fn advance(self) -> Self {
        let mut x = self.x;
        let mut y = self.y;

        match self.d {
            Direction::North => y += 1,
            Direction::West => x -= 1,
            Direction::South => y -= 1,
            Direction::East => x += 1,
        };
        Robot {
            x,
            y,
            d: self.d,
        }
    }

    pub fn instructions(self, instructions: &str) -> Self {
        let mut res = Robot {
            x: self.x,
            y: self.y,
            d: self.d,
        };

        for c in instructions.chars() {
            if c == 'A' {
                res = res.advance();
            }
            else if c == 'L' {
                res = res.turn_left();
            }
            else if c == 'R' {
                res = res.turn_right();
            }
        }
        res
    }

    pub fn position(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    pub fn direction(&self) -> &Direction {
        &self.d
    }
}
