use std::collections::HashMap;

pub struct Day14Part2;

const CYCLE: &[Direction] = &[
    Direction::North,
    Direction::West,
    Direction::South,
    Direction::East,
];

impl crate::days::Day for Day14Part2 {
    fn solve(&self, input: &str) -> String {
        let actual_cols = input.find('\n').expect("has line separator");
        let str_cols = actual_cols + 1;
        let rows = 1 + (input.len() / str_cols) as i32;

        let cubed_rock_vec: Vec<Location> = input.match_indices('#')
            .map(|(idx, _)| ((idx % str_cols) as i32, (idx / str_cols) as i32))
            .map(Location::from_tuple)
            .collect();

        // possibly optimizable:
        let cubed_rock_cols: Vec<Vec<Coordinate>> = (0..actual_cols)
            .map(|col| cubed_rock_vec.iter()
                .filter(|&rock_loc| rock_loc.col == col as i32)
                .map(|rock_loc| rock_loc.row)
                .collect()
            )
            .collect();

        let cubed_rock_rows: Vec<Vec<Coordinate>> = (0..rows)
            .map(|row| cubed_rock_vec.iter()
                .filter(|&rock_loc| rock_loc.row == row as i32)
                .map(|rock_loc| rock_loc.col)
                .collect()
            )
            .collect();

        let round_rock_vec: Vec<_> = input.match_indices('O')
            .map(|(idx, _)| ((idx % str_cols) as i32, (idx / str_cols) as i32))
            .map(Location::from_tuple)
            .collect();

        let mut cache: HashMap<Vec<Location>, i32> = HashMap::new();
        let mut round_rocks = round_rock_vec;
        
        let mut curr_cycle = 0;
        let (cycle_start, cycle_len) = 'find_cycle: loop {
            curr_cycle += 1;
            for tilt_direction in CYCLE {
                let cubed_rocks = tilt_direction.choose_cubed(&cubed_rock_cols, &cubed_rock_rows);
                let mut next_round_rocks = Vec::new();

                // iff idx1 = col, then idx2 = row (North, South) and vice versa (West, East)
                for (cubed_idx1, cubed_col_or_row) in cubed_rocks.iter().enumerate() {
                    let cubed_idx1 = cubed_idx1 as Coordinate;

                    // ugh. quick and dirty
                    // https://stackoverflow.com/questions/75976750/how-do-i-conditionally-iterate-in-reverse
                    let mut temp_iter1;
                    let mut temp_iter2;
                    let inner_iter: &mut dyn Iterator<Item = _> = if tilt_direction.needs_reversed() {
                        temp_iter1 = cubed_col_or_row.iter().rev();
                        &mut temp_iter1
                    } else {
                        temp_iter2 = cubed_col_or_row.iter();
                        &mut temp_iter2
                    };

                    for cubed_idx2 in inner_iter {
                        let round_rock_filter = tilt_direction.round_rock_filter(cubed_idx1, *cubed_idx2);
                        let old_len = round_rocks.len();
                        round_rocks.retain(round_rock_filter);
                        let moved_rock_count = old_len - round_rocks.len();

                        next_round_rocks.extend(tilt_direction.construct_moved_rocks(cubed_idx1, *cubed_idx2, moved_rock_count));
                    }

                    let bound = tilt_direction.get_bound(actual_cols as Coordinate, rows);
                    let round_rock_filter = tilt_direction.round_rock_filter(cubed_idx1, bound);
                    let old_len = round_rocks.len();
                    round_rocks.retain(round_rock_filter);
                    let moved_rock_count = old_len - round_rocks.len();
                    next_round_rocks.extend(tilt_direction.construct_moved_rocks(cubed_idx1, bound, moved_rock_count));
                }

                assert!(round_rocks.is_empty());
                round_rocks = next_round_rocks;
            }

            if let Some(&v) = cache.get(&round_rocks) {
                break 'find_cycle (v, curr_cycle - v);
            } else {
                cache.insert(round_rocks.clone(), curr_cycle);
            }
        };

        let rest_steps = (1_000_000_000 - cycle_start) % cycle_len;
        // get from hash map, avoid running the transformations again
        let round_rocks = cache.into_iter()
            .find(|(_, c)| *c == cycle_start + rest_steps)
            .map(|(map, _)| map)
            .unwrap();

        #[cfg(debug_assertions)]
        print_field(&round_rocks, &cubed_rock_vec, actual_cols as i32, rows);
        round_rocks.into_iter()
            .fold(0, |acc, rock| acc + rows - rock.row)
            .to_string()
    }
}

#[cfg(debug_assertions)]
fn print_field(round_rocks: &[Location], cubed_rocks: &[Location], cols: Coordinate, rows: Coordinate) {
    for row in 0..rows {
        let mut row_string = String::with_capacity(row as usize);
        for col in 0..cols {
            let next = if round_rocks.contains(&Location { col, row }) {
                'O'
            } else if cubed_rocks.contains(&Location { col, row }) {
                '#'
            } else {
                '.'
            };

            row_string.push(next);
        }
        println!("{row_string}");
    }
}

type Coordinate = i32;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Location {
    col: Coordinate,
    row: Coordinate,
}

impl Location {
    fn from_tuple(tuple: (i32, i32)) -> Self {
        Self {
            col: tuple.0,
            row: tuple.1,
        }
    }
}

enum Direction {
    North,
    West,
    South,
    East,
}

impl Direction {
    fn choose_cubed<'a>(&self, cols: &'a [Vec<Coordinate>], rows: &'a [Vec<Coordinate>]) -> &'a [Vec<i32>] {
        match self {
            Direction::North | Direction::South => cols,
            Direction::West | Direction::East => rows,
        }
    }

    fn needs_reversed(&self) -> bool {
        match self {
            Direction::North | Direction::West => true,
            Direction::South | Direction::East => false,
        }
    }

    fn round_rock_filter(&self, idx1: Coordinate, idx2: Coordinate) -> Box<dyn Fn(&Location) -> bool> {
        match self {
            Direction::North => Box::new(move |loc: &Location| !(loc.col == idx1 && loc.row > idx2)),
            Direction::South => Box::new(move |loc: &Location| !(loc.col == idx1 && loc.row < idx2)),
            Direction::West => Box::new(move |loc: &Location| !(loc.row == idx1 && loc.col > idx2)),
            Direction::East => Box::new(move |loc: &Location| !(loc.row == idx1 && loc.col < idx2)),
        }
    }

    fn get_bound(&self, cols: Coordinate, rows: Coordinate) -> Coordinate {
        match self {
            Direction::North | Direction::West => -1,
            Direction::South => rows,
            Direction::East => cols,
        }
    }

    fn construct_moved_rocks(&self, idx1: Coordinate, idx2: Coordinate, count: usize) -> Vec<Location> {
        (1..=count as Coordinate)
            .map(|c| {
                match self {
                    Direction::North => Location::from_tuple((idx1, idx2 + c)),
                    Direction::South => Location::from_tuple((idx1, idx2 - c)),
                    Direction::West => Location::from_tuple((idx2 + c, idx1)),
                    Direction::East => Location::from_tuple((idx2 - c, idx1)),
                }
            })
            .collect()
    }
}
