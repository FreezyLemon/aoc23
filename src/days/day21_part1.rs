use crate::cartesian::{Cartesian, Direction, Position};

const ALL_DIRECTIONS: &[Direction] = &[
    Direction::North,
    Direction::West,
    Direction::South,
    Direction::East,
];

pub struct Day21Part1;

impl crate::days::Day for Day21Part1 {
    fn solve(&self, input: &str) -> String {
        let cols = input.find('\n').expect("has line separator");
        let str_cols = 1 + cols;

        let starting_pos = input.find('S').expect("input has S");
        let starting_pos = Position {
            col: u32::try_from(starting_pos % str_cols).expect("fits into u32"),
            row: u32::try_from(starting_pos / str_cols).expect("fits into u32"),
        };

        let rocks: Vec<bool> = input.chars()
            .filter(|c| *c != '\n')
            .map(|c| c == '#')
            .collect();

        let rocks = Cartesian::new(
            rocks,
            u32::try_from(cols).expect("cols fits into u32"),
        );

        let mut positions = vec![starting_pos];
        for _ in 0..64 {
            let mut next_positions = Vec::new();
            for pos in positions {
                for direction in ALL_DIRECTIONS {
                    let Some(new_pos) = pos.in_direction(*direction) else {
                        continue;
                    };

                    let Some(is_rock) = rocks.get(new_pos) else {
                        continue;
                    };

                    if !is_rock {
                        next_positions.push(new_pos);
                    }
                }
            }

            next_positions.sort_unstable();
            next_positions.dedup();

            positions = next_positions;
        }
        
        positions.len().to_string()
    }
}
