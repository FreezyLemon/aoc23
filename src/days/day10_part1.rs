pub struct Day10Part1;

impl crate::days::Day for Day10Part1 {
    fn solve(&self, input: String) -> String {
        let tile_map: Vec<Vec<_>> = input.lines()
            .map(str::chars)
            .map(|chars| chars.map(Tile::from_char).collect())
            .collect();

        // println!("{tile_map:?}");

        let start_pos = tile_map.iter()
            .enumerate()
            .filter_map(|(y, row)| {
                row.iter()
                    .enumerate()
                    .find(|(_, &t)| t == Tile::Start)
                    .map(|(x, _)| Position { x: x as u32, y: y as u32 })
            })
            .nth(0)
            .expect("has starting position");

        let mut candidates = vec![
            (start_pos, Direction::North),
            (start_pos, Direction::East),
            (start_pos, Direction::South),
            (start_pos, Direction::West),
        ];

        let mut steps = 0;

        'outer: loop {
            let mut next_candidates = Vec::new();
            steps += 1;
            for (curr_pos, curr_direction) in candidates {
                let Some(new_pos) = curr_pos.in_direction(curr_direction) else {
                    continue;
                };

                let Some(new_tile) = get_tile_at(new_pos, &tile_map) else {
                    continue;
                };

                if new_tile == Tile::Start {
                    break 'outer;
                }

                if new_tile.connects_from(curr_direction.invert()) {
                    let new_direction = new_tile.other_direction(curr_direction.invert());
                    next_candidates.push((new_pos, new_direction));
                }
            }

            candidates = next_candidates;
        }

        (steps / 2).to_string()
    }
}

fn get_tile_at(pos: Position, tiles: &[Vec<Tile>]) -> Option<Tile> {
    tiles.get(pos.y as usize)
        .and_then(|row| row.get(pos.x as usize))
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
        let mut res = *self;

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
    East,
    South,
    West,
}

impl Direction {
    fn invert(self) -> Self {
        use Direction::*;
        match self {
            North => South,
            South => North,
            East => West,
            West => East,
        }
    }
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

    fn connects_from(self, direction: Direction) -> bool {
        use Tile::*;
        use Direction::*;

        match (self, direction) {
            (Ground, _) => false,
            (Start, _) => true,
            (Vertical, North | South) => true,
            (Horizontal, West | East) => true,
            (NorthEast, North | East) => true,
            (NorthWest, North | West) => true,
            (SouthEast, South | East) => true,
            (SouthWest, South | West) => true,
            _ => false,
        }
    }

    fn other_direction(self, direction: Direction) -> Direction {
        use Tile::*;
        use Direction::*;

        match (self, direction) {
            (Vertical, North) => South,
            (Vertical, South) => North,
            (Horizontal, West) => East,
            (Horizontal, East) => West,
            (NorthEast, North) => East,
            (NorthEast, East) => North,
            (NorthWest, North) => West,
            (NorthWest, West) => North,
            (SouthEast, South) => East,
            (SouthEast, East) => South,
            (SouthWest, South) => West,
            (SouthWest, West) => South,
            _ => panic!("invalid function call"),
        }
    }
}

