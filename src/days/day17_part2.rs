use std::{ops::{IndexMut, Index}, collections::BinaryHeap, cmp::Reverse};

pub struct Day17Part2;

impl crate::days::Day for Day17Part2 {
    fn solve(&self, input: &str) -> String {
        let cols = input.find('\n').expect("has line separator");

        let costs: Vec<u8> = input.chars()
            .filter_map(|c| c.to_digit(10).map(|u| u as u8))
            .collect();

        let cost_map = CostMap { data: costs, cols: u32::try_from(cols).unwrap() };
        dijkstra(&cost_map, (0, 0)).to_string()
    }
}

fn dijkstra(cost_map: &CostMap, start_at: (u32, u32)) -> u32 {
    let mut distance_map = Cartesian {
        data: vec![Vec::new(); cost_map.data.len()],
        cols: cost_map.cols,
    };

    let start_idx = start_at;
    let end_idx = (cost_map.cols - 1, cost_map.rows() - 1);
    let mut end_cost = u32::MAX;

    // can't go north from start, so this is fine
    distance_map[start_idx] = vec![(0u32, Direction::North, 0)];

    let mut queue = BinaryHeap::new();
    queue.push((Reverse(0u32), Reverse(0), start_idx, Direction::East));

    while let Some((Reverse(cost), Reverse(prev_direction_count), idx, prev_direction)) = queue.pop() {
        if cost > end_cost {
            break;
        }

        for (neighbour, new_direction) in get_neighbours(idx, prev_direction, prev_direction_count, cost_map.cols, cost_map.rows()) {
            let new_cost = cost + u32::from(cost_map[neighbour]);

            if new_cost >= end_cost {
                continue;
            }

            let new_direction_count = if new_direction == prev_direction {
                prev_direction_count + 1
            } else {
                1
            };

            if neighbour == end_idx && new_direction_count >= 4 {
                end_cost = new_cost;
                continue;
            }

            let neighbour_distances = &mut distance_map[neighbour];
            if neighbour_distances.iter().all(|(_, direction, _)| new_direction != *direction) {
                // never visited in this direction before. just add and go on
                neighbour_distances.push((new_cost, new_direction, new_direction_count));
                queue.push((Reverse(new_cost), Reverse(new_direction_count), neighbour, new_direction));
                continue;
            }

            if let Some(entry) = neighbour_distances.iter_mut().find(|(_, direction, direction_count)| *direction == new_direction && *direction_count == new_direction_count) {
                if new_cost < entry.0 {
                    entry.0 = new_cost;
                    queue.push((Reverse(new_cost), Reverse(new_direction_count), neighbour, new_direction));
                    continue;
                }
            } else {
                neighbour_distances.push((new_cost, new_direction, new_direction_count));
                queue.push((Reverse(new_cost), Reverse(new_direction_count), neighbour, new_direction));
            }
        }
    }

    end_cost
}

fn get_neighbours(idx: (u32, u32), prev_direction: Direction, prev_direction_count: u32, cols: u32, rows: u32) -> Vec<((u32, u32), Direction)> {
    let (col, row) = idx;
    let mut result = Vec::with_capacity(4);
    if col > 0  && prev_direction != Direction::East {
        result.push(((col - 1, row), Direction::West));
    }
    if col < cols - 1 && prev_direction != Direction::West {
        result.push(((col + 1, row), Direction::East));
    }
    if row > 0 && prev_direction != Direction::South {
        result.push(((col, row - 1), Direction::North));
    }
    if row < rows - 1 && prev_direction != Direction::North {
        result.push(((col, row + 1), Direction::South));
    }

    if prev_direction_count > 0 && prev_direction_count < 4 {
        result.retain(|(_, direction)| *direction == prev_direction);
    }

    if prev_direction_count >= 10 {
        result.retain(|(_, direction)| *direction != prev_direction);
    }

    result
}

type CostMap = Cartesian<u8>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Direction {
    North = 4,
    West = 3,
    South = 2,
    East = 1,
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
