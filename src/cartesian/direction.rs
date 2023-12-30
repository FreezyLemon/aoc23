#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    // uncomment when needed
    // pub fn turn_left(self) -> Self {
    //     use Direction::*;
    //     match self {
    //         North => West,
    //         East => North,
    //         South => East,
    //         West => South,
    //     }
    // }

    // pub fn invert(self) -> Self {
    //     self.turn_left().turn_left()
    // }

    // pub fn turn_right(self) -> Self {
    //     self.turn_left().turn_left().turn_left()
    // }
}
