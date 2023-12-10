pub struct Day10Part2;

impl crate::days::Day for Day10Part2 {
    fn solve(&self, input: String) -> String {
        let tile_map: Vec<Vec<_>> = input.lines()
            .map(str::chars)
            .map(|chars| chars.map(Tile::from_char).collect())
            .collect();

        // println!("{tile_map:?}");

        // starting position might have to be replaced by a correct pipe

        let rows = tile_map.len();
        let cols = tile_map[0].len();
        let ground_positions: Vec<_> = tile_map.iter()
            .enumerate()
            .flat_map(|(y, row)| {
                row.iter()
                    .enumerate()
                    .filter(|(_, &tile)| tile == Tile::Ground)
                    .map(move |(x, _)| {
                        (
                            Position { x: x as u32, y: y as u32 },
                            EnclosedState::Unknown,
                        )
                    })
            })
            .collect();

        // find all connected ground tiles
        // check if connected to outside border
        // repeat until no unknowns

        // take one ground tile
        let (candidate_pos, _) = ground_positions.iter()
            .filter(|(_, s)| s == &EnclosedState::Unknown)
            .nth(1)
            .unwrap();

        // find all connected tiles
        // repeat for all 8 directions:
        // 1. get tile in direction
        // 2. if ground: add to partition, if not already
        // 2. else if not diagonal: ignore
        // 2. if diagonal: check further
        // 3. follow pipe
        

        "".into()
    }
}

fn get_tile_at(pos: Position, tiles: &Vec<Vec<Tile>>) -> Option<Tile> {
    tiles.get(pos.y as usize)
        .map(|row| row.get(pos.x as usize))
        .flatten()
        .copied()
}

#[derive(Debug, Clone, Copy)]
// x, y
struct Position {
    x: u32,
    y: u32,
}

impl Position {
    fn in_direction(&self, d: Direction) -> Option<Self> {
        let mut res = self.clone();

        match d {
            Direction::North if res.y > 0 => res.y -= 1,
            Direction::South => res.y += 1,
            Direction::West if res.x > 0 => res.x -= 1,
            Direction::East => res.x += 1,
            _ => return None,
        }

        Some(res)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

//impl Direction {
//    fn invert(self) -> Self {
//        use Direction::*;
//        match self {
//            North => South,
//            South => North,
//            East => West,
//            West => East,
//        }
//    }
//}

#[derive(PartialEq, Eq)]
enum EnclosedState {
    Unknown,
    Outside,
    Inside,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Vertical,
    Horizontal,
    NorthEast,
    NorthWest,
    SouthEast,
    SouthWest,
    Ground,
    Start,
}

impl Tile {
    fn from_char(c: char) -> Self {
        match c {
            '|' => Tile::Vertical,
            '-' => Tile::Horizontal,
            'L' => Tile::NorthEast,
            'J' => Tile::NorthWest,
            'F' => Tile::SouthEast,
            '7' => Tile::SouthWest,
            '.' => Tile::Ground,
            'S' => Tile::Start,
            _ => panic!("invalid tile char"),
        }
    }

//    fn connects_from(self, direction: Direction) -> bool {
//        use Tile::*;
//        use Direction::*;
//
//        match (self, direction) {
//            (Ground, _) => false,
//            (Start, _) => true,
//            (Vertical, North | South) => true,
//            (Horizontal, West | East) => true,
//            (NorthEast, North | East) => true,
//            (NorthWest, North | West) => true,
//            (SouthEast, South | East) => true,
//            (SouthWest, South | West) => true,
//            _ => false,
//        }
//    }
//
//    fn other_direction(self, direction: Direction) -> Direction {
//        use Tile::*;
//        use Direction::*;
//
//        match (self, direction) {
//            (Vertical, North) => South,
//            (Vertical, South) => North,
//            (Horizontal, West) => East,
//            (Horizontal, East) => West,
//            (NorthEast, North) => East,
//            (NorthEast, East) => North,
//            (NorthWest, North) => West,
//            (NorthWest, West) => North,
//            (SouthEast, South) => East,
//            (SouthEast, East) => South,
//            (SouthWest, South) => West,
//            (SouthWest, West) => South,
//            _ => panic!("invalid function call"),
//        }
//    }
}

