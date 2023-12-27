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
        let end_idx = (cost_map.cols - 1, cost_map.rows() - 1);
        let (distances, _path) = dijkstra(&cost_map, (0, 0));

        distances[end_idx].iter().map(|(c, _, _)| c).min().unwrap().to_string()
    }
}

// fn a_star(cost_map: CostMap) -> u32 {
//     let mut distance_map = Cartesian {
//         data: vec![u32::MAX; cost_map.data.len()],
//         cols: cost_map.cols,
//     };
//     let mut prev_map = Cartesian {
//         data: vec![None; cost_map.data.len()],
//         cols: cost_map.cols,
//     };

//     let start_idx = (0, 0);
//     let end_idx = (cost_map.cols - 1, cost_map.rows() - 1);

//     *&mut distance_map[start_idx] = cost_map[start_idx] as u32;
//     // can't go north from start, so this is fine
//     prev_map[start_idx] = Some((start_idx, Direction::North, 0));

//     let mut queue = BinaryHeap::new();
//     queue.push((Reverse(manhattan_distance(end_idx, start_idx) + cost_map[start_idx] as u32), start_idx));

//     while let Some((_, idx)) = queue.pop() {
//         // if idx == end_idx {
//         //     break;
//         // }

//         // if p != manhattan_distance(end_idx, idx) + distance_map[idx] {
//             // the distance has changed since we pushed this to the queue
//             // -> we've already visited this index in a prior iteration
//             // continue;
//         // }

//         for (neighbour, direction) in get_neighbours(idx, cost_map.cols, cost_map.rows()) {
//             let alt = distance_map[idx] + cost_map[neighbour] as u32;

//             if alt < distance_map[neighbour] {
//                 let (_, prev_direction, same_direction_count) = prev_map[idx].unwrap();

//                 let next_direction_count = if direction == prev_direction {
//                     same_direction_count + 1
//                 } else {
//                     1
//                 };

//                 if next_direction_count > 3 {
//                     continue;
//                 }

//                 distance_map[neighbour] = alt;
//                 prev_map[neighbour] = Some((idx, direction, next_direction_count));
//                 queue.push((Reverse(alt + manhattan_distance(end_idx, neighbour)), neighbour));
//             } else if alt == distance_map[neighbour] {
//                 let (_, prev_direction, same_direction_count) = prev_map[idx].unwrap();
//                 // this implies distance_map[neighbour] < u32::MAX,
//                 // so it has been visited before
//                 let (_, nb_prev_direction, nb_same_direction_count) = prev_map[neighbour].unwrap();

//                 // go further for the same distance, if possible
//                 if nb_same_direction_count > 1 + same_direction_count {
//                     let next_direction_count = if direction == prev_direction {
//                         same_direction_count + 1
//                     } else {
//                         1
//                     };

//                     if next_direction_count > 3 {
//                         continue;
//                     }

//                     prev_map[neighbour] = Some((idx, direction, next_direction_count));
//                     queue.push((Reverse(alt + manhattan_distance(end_idx, neighbour)), neighbour));
//                 } 
//             }
//         }
//     }

//     let mut curr_entry = prev_map[end_idx].unwrap();
//     let mut path = vec![curr_entry];
//     while curr_entry.0 != start_idx {
//         let next_entry = prev_map[curr_entry.0].unwrap();
//         path.push(next_entry);
//         curr_entry = next_entry;
//     }

//     let path: Vec<_> = path.into_iter().rev().map(|(idx, _, _)| idx).collect();

//     println!("{path:?}");

//     distance_map[end_idx]
// }

// fn manhattan_distance(to: (u32, u32), from: (u32, u32)) -> u32 {
//     u32::try_from((to.0 as i32 - from.0 as i32).abs() + (to.1 as i32 - from.1 as i32).abs()).expect("distance is positive")
// }

fn dijkstra(cost_map: &CostMap, start_at: (u32, u32)) -> (Cartesian<Vec<(u32, Direction, u32)>>, Vec<(u32, u32)>) {
    let mut distance_map = Cartesian {
        data: vec![Vec::new(); cost_map.data.len()],
        cols: cost_map.cols,
    };
    let mut prev_map = Cartesian {
        data: vec![None; cost_map.data.len()],
        cols: cost_map.cols,
    };

    let start_idx = start_at;
    //let end_idx = (cost_map.cols - 1, cost_map.rows() - 1);

    distance_map[start_idx] = vec![(cost_map[start_idx] as u32, Direction::North, 0)];
    // can't go north from start, so this is fine
    prev_map[start_idx] = Some(start_idx);

    let mut queue = BinaryHeap::new();
    queue.push((Reverse(cost_map[start_idx] as u32), Reverse(0), start_idx, Direction::North));

    while let Some((Reverse(cost), Reverse(prev_direction_count), idx, prev_direction)) = queue.pop() {
        // if idx == end_idx {
        //     break;
        // }

        // if p > distance_map[idx] {
            // the distance has changed since we pushed this to the queue
            // -> we've already visited this index in a prior iteration
            // continue;
        // }

        for (neighbour, new_direction) in get_neighbours(idx, prev_direction, prev_direction_count, cost_map.cols, cost_map.rows()) {
            let new_cost = cost + cost_map[neighbour] as u32;
            let new_direction_count = if new_direction == prev_direction {
                prev_direction_count + 1
            } else {
                1
            };

            let neighbour_distances = &mut distance_map[neighbour];
            if neighbour_distances.iter().all(|(_, direction, _)| new_direction != *direction) {
                // never visited in this direction before. just add and go on
                prev_map[neighbour] = Some(idx);
                neighbour_distances.push((new_cost, new_direction, new_direction_count));
                queue.push((Reverse(new_cost), Reverse(new_direction_count), neighbour, new_direction));
                continue;
            }

            let neighbour_distance_count = neighbour_distances.len();
            neighbour_distances.retain(|(cost, direction, direction_count)| *direction != new_direction || *cost < new_cost || *direction_count < new_direction_count);
            if neighbour_distances.len() < neighbour_distance_count {
                prev_map[neighbour] = Some(idx);
                neighbour_distances.push((new_cost, new_direction, new_direction_count));
                queue.push((Reverse(new_cost), Reverse(new_direction_count), neighbour, new_direction));
                continue;
            }

            if let Some((_, _, min_neighbour_dir_count)) = neighbour_distances.iter().filter(|(_, direction, _)| new_direction == *direction).min_by_key(|(_, _, c)| c) {
                if new_direction_count < *min_neighbour_dir_count {
                    prev_map[neighbour] = Some(idx);
                    neighbour_distances.push((new_cost, new_direction, new_direction_count));
                    queue.push((Reverse(new_cost), Reverse(new_direction_count), neighbour, new_direction));
                }
            }

            // let alt = distance_map[idx] + cost_map[neighbour] as u32;
            // if alt < distance_map[neighbour] {
            //     let (_, prev_direction, same_direction_count) = prev_map[idx].unwrap();

            //     let next_direction_count = if new_direction == prev_direction {
            //         same_direction_count + 1
            //     } else {
            //         1
            //     };

            //     if next_direction_count > 3 {
            //         continue;
            //     }

            //     distance_map[neighbour] = alt;
            //     prev_map[neighbour] = Some((idx, new_direction, next_direction_count));
            //     queue.push((Reverse(alt), neighbour));
            // }
        }
    }

    // let mut curr_idx = prev_map[end_idx].unwrap();
    // let mut path = vec![curr_idx];
    // while curr_idx != start_idx {
    //     let next_entry = prev_map[curr_idx].unwrap();
    //     path.push(next_entry);
    //     curr_idx = next_entry;
    // }

    // let path: Vec<_> = path.into_iter().rev().collect();

    (distance_map, vec![])
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

    if prev_direction_count >= 3 {
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
