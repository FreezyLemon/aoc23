use std::{ops::{IndexMut, Index}, collections::BinaryHeap, cmp::Reverse};

pub struct Day17Part1;

impl crate::days::Day for Day17Part1 {
    fn solve(&self, input: &str) -> String {
        let cols = input.find('\n').expect("has line separator");
        // let str_cols = LINE_SEPARATOR.len() + cols;
        // let rows = 1 + (input.len() / str_cols);

        let costs: Vec<u8> = input.chars()
            .filter_map(|c| c.to_digit(10).map(|u| u as u8))
            .collect();

        let cost_map = CostMap { data: costs, cols: u32::try_from(cols).unwrap() };
        dijkstra(cost_map).to_string()
    }
}

fn dijkstra(cost_map: CostMap) -> u32 {
    let mut distance_map = Cartesian {
        data: vec![u32::MAX; cost_map.data.len()],
        cols: cost_map.cols,
    };
    let mut prev_map = Cartesian {
        data: vec![None; cost_map.data.len()],
        cols: cost_map.cols,
    };

    let start_idx = (0, 0);
    let end_idx = (cost_map.cols - 1, cost_map.rows() - 1);

    *&mut distance_map[start_idx] = cost_map[start_idx] as u32;

    let mut queue = BinaryHeap::new();
    queue.push((Reverse(cost_map[start_idx] as u32), start_idx));

    while let Some((Reverse(p), idx)) = queue.pop() {
        if p != distance_map[idx] {
            continue;
        }

        for (neighbour, direction) in get_neighbours(idx, cost_map.cols, cost_map.rows()) {
            let alt = distance_map[idx] + cost_map[neighbour] as u32;
            if alt < distance_map[neighbour] {
                distance_map[neighbour] = alt;
                let same_direction_count = prev_map[neighbour]
                    .map_or(0, |(_, d, dc)| {
                        if d == direction {
                            dc
                        } else {
                            0
                        }
                    });
                
                if same_direction_count == 3 {
                    // can't go further than 3 blocks before turning
                    continue;
                }

                prev_map[neighbour] = Some((idx, direction, same_direction_count + 1));
                queue.push((Reverse(alt), neighbour));
            }
        }
    }

    // TODO: print prev_map so we can see the path here

    distance_map[end_idx]
}

fn get_neighbours(idx: (u32, u32), cols: u32, rows: u32) -> Vec<((u32, u32), Direction)> {
    let (col, row) = idx;
    let mut result = Vec::with_capacity(4);
    if col > 0 {
        result.push(((col - 1, row), Direction::West));
    }
    if col < cols - 1 {
        result.push(((col + 1, row), Direction::East));
    }
    if row > 0 {
        result.push(((col, row - 1), Direction::North));
    }
    if row < rows - 1 {
        result.push(((col, row + 1), Direction::South));
    }
    result
}

type CostMap = Cartesian<u8>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    North,
    West,
    South,
    East,
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
