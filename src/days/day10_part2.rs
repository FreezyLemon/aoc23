pub struct Day10Part2;

const ALL_DIRECTIONS: &[Direction] = &[
    Direction::North,
    Direction::East,
    Direction::South,
    Direction::West,
];

impl crate::days::Day for Day10Part2 {
    fn solve(&self, input: String) -> String {
        let tile_map: Vec<Vec<_>> = input.lines()
            .map(str::chars)
            .map(|chars| chars.map(Tile::from_char).collect())
            .collect();

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

        let first = ALL_DIRECTIONS.iter()
            .filter_map(|&direction| {
                start_pos
                    .in_direction(direction)
                    .and_then(|new_pos| get_tile_at(new_pos, &tile_map).map(|t| (new_pos, t)))
                    .filter(|(_, tile)| tile.connects_from(direction.invert()))
                    .map(|(new_pos, tile)| (new_pos, direction, tile.other_direction(direction.invert())))
            })
            .nth(0)
            .expect("can find pipe connected to start");

        let mut loop_path = vec![start_pos];
        let mut left_partition = Vec::new();
        let mut right_partition = Vec::new();

        let mut curr_search = first;
        while curr_search.0 != start_pos {
            let (curr_pos, arrival_dir, curr_dir) = curr_search;
            loop_path.push(curr_pos);

            // These two should never fail for the main loop
            let next_pos = curr_pos.in_direction(curr_dir).unwrap();
            let next_tile = get_tile_at(next_pos, &tile_map).unwrap();

            debug_assert!(next_tile.connects_from(curr_dir.invert()));

            if let Some(left_pos) = curr_pos.in_direction(arrival_dir.turn_left()) {
                if get_tile_at(left_pos, &tile_map).is_some() {
                    left_partition.push(left_pos);
                }
            }

            if let Some(left_pos) = curr_pos.in_direction(curr_dir.turn_left()) {
                if get_tile_at(left_pos, &tile_map).is_some() {
                    left_partition.push(left_pos);
                }
            }

            if let Some(right_pos) = curr_pos.in_direction(arrival_dir.turn_right()) {
                if get_tile_at(right_pos, &tile_map).is_some() {
                    right_partition.push(right_pos);
                }
            }

            if let Some(right_pos) = curr_pos.in_direction(curr_dir.turn_right()) {
                if get_tile_at(right_pos, &tile_map).is_some() {
                    right_partition.push(right_pos);
                }
            }

            curr_search = (next_pos, curr_dir, next_tile.other_direction(curr_dir.invert()));
        }

        // The partitions are not complete yet.
        // We remove duplicates and move on to finding all adjacent non-loop tiles
        loop_path.sort_unstable();

        left_partition.sort_unstable();
        left_partition.dedup();
        let left_partition: Vec<Position> = left_partition.into_iter()
            .filter(|pos| loop_path.binary_search(pos).is_err())
            .collect();
        // println!("left: {}", left_partition.len());

        right_partition.sort_unstable();
        right_partition.dedup();
        let right_partition: Vec<Position> = right_partition.into_iter()
            .filter(|pos| loop_path.binary_search(pos).is_err())
            .collect();

        let (left_is_outside, left_partition) = complete_partition(left_partition, &tile_map, &loop_path);
        let (right_is_outside, right_partition) = complete_partition(right_partition, &tile_map, &loop_path);

        // println!("left: {}", left_partition.len());
        // println!("right: {}", right_partition.len());
        //println!("left outside: {left_is_outside}");
        assert!(left_is_outside != right_is_outside);

        let inside_tile_count = if left_is_outside {
            right_partition.len()
        } else {
            left_partition.len()
        };

        inside_tile_count.to_string()
    }
}

fn complete_partition(partition: Vec<Position>, tiles: &Vec<Vec<Tile>>, loop_path: &[Position]) -> (bool, Vec<Position>) {
    if partition.is_empty() {
        return (true, partition);
    }

    let mut search = partition.clone();
    let mut visited = partition.clone();
    let mut result = partition;
    let mut is_outside = false;

    while !search.is_empty() {
        let mut next_search = Vec::new();

        for pos in search {
            for &dir in ALL_DIRECTIONS {
                let Some(next_pos) = pos.in_direction(dir) else {
                    continue;
                };

                if visited.contains(&next_pos) {
                    continue;
                }

                if loop_path.binary_search(&next_pos).is_ok() {
                    continue;
                }

                if get_tile_at(next_pos, tiles).is_none() {
                    continue;
                };

                if next_pos.x == 0 || next_pos.y == 0 ||
                    next_pos.x as usize == tiles[0].len() ||
                    next_pos.y as usize == tiles.len()
                {
                    is_outside = true;
                }

                visited.push(next_pos);
                result.push(next_pos);
                next_search.push(next_pos);
            }
        }

        // println!("added {} tiles, {} total", next_search.len(), result.len());
        search = next_search;
    }

    (is_outside, result)
}

fn get_tile_at(pos: Position, tiles: &[Vec<Tile>]) -> Option<Tile> {
    tiles.get(pos.y as usize)
        .and_then(|row| row.get(pos.x as usize))
        .copied()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
    fn turn_left(self) -> Self {
        use Direction::*;
        match self {
            North => West,
            East => North,
            South => East,
            West => South,
        }
    }

    fn invert(self) -> Self {
        self.turn_left().turn_left()
    }

    fn turn_right(self) -> Self {
        self.turn_left().turn_left().turn_left()
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
            (Start, d) => d.invert(), // shouldn't really matter what we return here
            (s, o) => panic!("{s:?} does not have a direction other than {o:?}"),
        }
    }
}

