use std::collections::HashMap;

pub struct Day8Part2;

impl crate::days::Day for Day8Part2 {
    fn solve(&self, input: String) -> String {
        let mut lines_iter = input.lines();
        let instructions: Vec<Direction> = lines_iter.next().expect("has at least one line")
            .chars()
            .map(|c| match c {
                'R' => Direction::Right,
                'L' => Direction::Left,
                _ => panic!("invalid input"),
            })
        .collect();
        
        lines_iter.next().unwrap(); // ignore empty line
        
        let path_map: HashMap<String, Node> = lines_iter.map(|l| l.split_once('=').expect("line has ="))
            .map(|(name, paths)| {
                let name = name.trim().to_string();
                let (l, r) = paths.trim().split_once(',').expect("line has ,");
                
                (
                    name.clone(),
                    Node {
                        _name: name,
                        left: l.trim_start_matches('(').to_string(),
                        right: r.trim().trim_end_matches(')').to_string(),
                    }
                )
            })
            .collect();

        let end_steps_per_node: Vec<i64> = path_map
            .iter()
            .filter(|(name, _)| name.ends_with('A'))
            .map(|(_name, node)| {
                let mut steps = 0;
                let mut curr_node = node;
                for dir in instructions.iter().cycle() {
                    let next_node_name = match dir {
                        Direction::Left => &curr_node.left,
                        Direction::Right => &curr_node.right,
                    };

                    steps += 1;

                    if next_node_name.ends_with('Z') {
                        break;
                    }
                    curr_node = path_map.get(next_node_name).unwrap();
                }

                steps
            })
            .collect();

        lcm(end_steps_per_node).to_string()
    }
}

fn lcm(mut ints: Vec<i64>) -> i64 {
    assert!(ints.len() > 1);

    let orig = ints.clone();
    loop {
        let v = ints[0];
        if ints.iter().all(|&x| x == v) {
            return v;
        }

        let (idx, _) = ints.iter()
            .enumerate()
            .min_by_key(|(_, &val)| val)
            .unwrap();

        ints[idx] += orig[idx];
    }
}

#[derive(Debug)]
struct Node {
    _name: String,
    left: String,
    right: String,
}

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

