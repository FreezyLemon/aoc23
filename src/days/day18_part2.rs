pub struct Day18Part2;

const ALL_DIRECTIONS: &[Direction] = &[
    Direction::North,
    Direction::East,
    Direction::South,
    Direction::West,
];

impl crate::days::Day for Day18Part2 {
    fn solve(&self, input: &str) -> String {
        let instructions: Vec<_> = input.lines()
            .map(|line| line.split_once(' ').expect("line has whitespace"))
            .map(|(_, line)| line.split_once(' ').expect("line has another whitespace"))
            .map(|(_, line)| line.trim_start_matches("(#").trim_end_matches(")").split_at(5))
            .map(|(hex, dir_code)| (Direction::parse(dir_code), u32_from_hex(hex)))
            .collect();

        println!("{instructions:?}");

        // offset from 0, 0 to allow movements in all directions
        let mut curr_pos = Position { x: 1000, y: 1000 };
        let mut walls = Vec::new();
        let mut left_partition = Vec::new();
        let mut right_partition = Vec::new();

        for (direction, steps) in instructions {
            for _ in 0..steps {
                curr_pos = curr_pos.in_direction(direction).expect("can move here");
                walls.push(curr_pos);

                if let Some(left_pos) = curr_pos.in_direction(direction.turn_left()) {
                    if !walls.contains(&left_pos) {
                        left_partition.push(left_pos);
                    }
                }
    
                if let Some(right_pos) = curr_pos.in_direction(direction.turn_right()) {
                    if !walls.contains(&right_pos) {
                        right_partition.push(right_pos);
                    }
                }
            }
        }

        // The partitions are not complete yet.
        // We remove duplicates and move on to finding all adjacent non-loop tiles
        walls.sort_unstable();

        left_partition.sort_unstable();
        left_partition.dedup();
        let left_partition: Vec<Position> = left_partition.into_iter()
            .filter(|pos| walls.binary_search(pos).is_err())
            .collect();

        right_partition.sort_unstable();
        right_partition.dedup();
        let right_partition: Vec<Position> = right_partition.into_iter()
            .filter(|pos| walls.binary_search(pos).is_err())
            .collect();

        let min_x = walls.iter().map(|w| w.x).min().unwrap();
        let max_x = walls.iter().map(|w| w.x).max().unwrap();
        let min_y = walls.iter().map(|w| w.y).min().unwrap();
        let max_y = walls.iter().map(|w| w.y).max().unwrap();

        let (left_is_outside, left_partition) = complete_partition(left_partition, min_x, max_x, min_y, max_y, &walls);
        let (right_is_outside, right_partition) = complete_partition(right_partition, min_x, max_x, min_y, max_y, &walls);

        assert!(left_is_outside != right_is_outside);

        let inside_tile_count = if left_is_outside {
            right_partition.len()
        } else {
            left_partition.len()
        };

        (walls.len() + inside_tile_count).to_string()
    }
}

fn u32_from_hex(hex: &str) -> u32 {
    assert_eq!(hex.len(), 5);

    u32::from_str_radix(hex, 16).expect("hex can be parsed")
}

fn complete_partition(partition: Vec<Position>, min_x: u32, max_x: u32, min_y: u32, max_y: u32, walls: &[Position]) -> (bool, Vec<Position>) {
    if partition.is_empty() {
        return (true, partition);
    }

    let mut search = partition.clone();
    let mut visited = partition.clone();
    visited.sort_unstable();
    let mut result = partition;

    while !search.is_empty() {
        let mut next_search = Vec::new();

        for pos in search {
            for &dir in ALL_DIRECTIONS {
                let Some(next_pos) = pos.in_direction(dir) else {
                    continue;
                };

                let Err(visited_new_pos) = visited.binary_search(&next_pos) else {
                    continue;
                };

                if walls.binary_search(&next_pos).is_ok() {
                    continue;
                }

                if next_pos.x < min_x ||
                    next_pos.y < min_y ||
                    next_pos.x > max_x ||
                    next_pos.y > max_y
                {
                    return (true, result);
                }

                visited.insert(visited_new_pos, next_pos);
                result.push(next_pos);
                next_search.push(next_pos);
            }
        }

        search = next_search;
    }

    (false, result)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
    fn parse(s: &str) -> Self {
        match s {
            "3" => Self::North,
            "2" => Self::West,
            "1" => Self::South,
            "0" => Self::East,
            s => panic!("unsupported input '{s}'"),
        }
    }

    fn turn_left(self) -> Self {
        use Direction::*;
        match self {
            North => West,
            East => North,
            South => East,
            West => South,
        }
    }

    // fn invert(self) -> Self {
    //     self.turn_left().turn_left()
    // }

    fn turn_right(self) -> Self {
        self.turn_left().turn_left().turn_left()
    }
}
