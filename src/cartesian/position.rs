use super::Direction;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Position {
    pub col: u32,
    pub row: u32,
}

impl Position {
    pub fn in_direction(&self, d: Direction) -> Option<Self> {
        let mut res = *self;

        match d {
            Direction::North if res.row > 0 => res.row -= 1,
            Direction::South => res.row += 1,
            Direction::West if res.col > 0 => res.col -= 1,
            Direction::East => res.col += 1,
            _ => return None,
        }

        Some(res)
    }
}
