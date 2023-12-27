use core::ops::{Index, IndexMut};
use std::{collections::BinaryHeap, cmp::Reverse};

pub struct Day17Part1;

impl crate::days::Day for Day17Part1 {
    fn solve(&self, input: &str) -> String {
        let cols = input.find('\n')
            .map(u32::try_from)
            .map(Result::ok)
            .flatten()
            .expect("has line separator that fits into u32");

        let str_cols = 1 + cols;
        let rows = 1 + u32::try_from(input.len() / str_cols as usize).expect("rows fits into u32");

        let costs: Vec<u8> = input.chars()
            .filter_map(|c| c.to_digit(10).map(|u| u as u8))
            .collect();

        let start_idx = (0, 0);
        let end_idx = (cols - 1, rows - 1);

        let cost_map = Cartesian {
            data: costs,
            cols,
        };

        let mut end_cost = u32::MAX;

        let mut queue = BinaryHeap::new();
        queue.push((Reverse(manhattan_distance(end_idx, start_idx)), Reverse(0u32), start_idx, vec![], Direction::North, 0));

        while let Some((Reverse(md), Reverse(cost), curr_idx, curr_path, curr_direction, curr_direction_count)) = queue.pop() {
            for (neighbour_idx, next_direction) in get_neighbours(curr_idx, curr_direction, curr_direction_count, cols, rows) {
                if md + cost > end_cost {
                    // we can't possibly beat the current best end_cost
                    continue;
                }

                let next_cost = cost + u32::from(cost_map[neighbour_idx]);

                if next_cost >= end_cost {
                    // no reason to continue this path
                    continue;
                }

                if neighbour_idx == end_idx {
                    println!("end_cost lowered to {next_cost}");
                    end_cost = next_cost;
                    continue;
                }

                if curr_path.contains(&neighbour_idx) {
                    continue;
                }

                let next_direction_count = if next_direction == curr_direction {
                    curr_direction_count + 1
                } else {
                    1
                };

                let mut next_path = curr_path.clone();
                next_path.push(neighbour_idx);
                queue.push((Reverse(manhattan_distance(end_idx, neighbour_idx)), Reverse(next_cost), neighbour_idx, next_path, next_direction, next_direction_count));
            }
        }

        end_cost.to_string()
    }
}

fn manhattan_distance(to: (u32, u32), from: (u32, u32)) -> u32 {
    u32::try_from((to.0 as i32 - from.0 as i32).abs() + (to.1 as i32 - from.1 as i32).abs()).expect("distance is positive")
}

fn get_neighbours(idx: (u32, u32), prev_direction: Direction, prev_direction_count: u32, cols: u32, rows: u32) -> Vec<((u32, u32), Direction)> {
    let (col, row) = idx;
    let mut result = Vec::with_capacity(4);
    if col < cols - 1 && prev_direction != Direction::West {
        result.push(((col + 1, row), Direction::East));
    }
    if col > 0  && prev_direction != Direction::East {
        result.push(((col - 1, row), Direction::West));
    }
    if row < rows - 1 && prev_direction != Direction::North {
        result.push(((col, row + 1), Direction::South));
    }
    if row > 0 && prev_direction != Direction::South {
        result.push(((col, row - 1), Direction::North));
    }
    if prev_direction_count >= 3 {
        result.retain(|(_, direction)| *direction != prev_direction);
    }

    result
}

struct Cartesian<T> {
    data: Vec<T>,
    cols: u32,
}

impl<T> Cartesian<T> {
    fn rows(&self) -> u32 {
        self.data.len() as u32 / self.cols
    }

    fn get(&self, col: u32, row: u32) -> Option<&T> {
        if col >= self.cols {
            None
        } else if row >= self.rows() {
            None
        } else {
            Some(&self.data[usize::try_from(row * self.cols + col).unwrap()])
        }
    }

    fn get_mut(&mut self, col: u32, row: u32) -> Option<&mut T> {
        if col >= self.cols {
            None
        } else if row >= self.rows() {
            None
        } else {
            Some(&mut self.data[usize::try_from(row * self.cols + col).unwrap()])
        }
    }
}

impl<T> Index<(u32, u32)> for Cartesian<T> {
    type Output = T;

    fn index(&self, index: (u32, u32)) -> &Self::Output {
        self.get(index.0, index.1).expect("is within bounds")
    }
}

impl<T> IndexMut<(u32, u32)> for Cartesian<T> {
    fn index_mut(&mut self, index: (u32, u32)) -> &mut Self::Output {
        self.get_mut(index.0, index.1).expect("is within bounds")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Direction {
    North,
    East,
    South,
    West,
}

