use std::collections::HashMap;

pub struct Day8Part1;

impl crate::days::Day for Day8Part1 {
    fn solve(&self, input: &str) -> String {
        let mut lines_iter = input.lines();
        let instructions = lines_iter.next().expect("has at least one line")
            .chars()
            .map(|c| match c {
                'R' => Direction::Right,
                'L' => Direction::Left,
                _ => panic!("invalid input"),
            });
        
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

        let mut curr_node_name = "AAA";
        let mut steps = 0;

        for instr in instructions.cycle() {
            let curr_node = path_map.get(curr_node_name).expect("node exists");

            let next_node_name = match instr {
                Direction::Left => &curr_node.left,
                Direction::Right => &curr_node.right,
            };
            curr_node_name = next_node_name;
            steps += 1;

            if curr_node_name == "ZZZ" {
                break;
            }
        }


        steps.to_string()
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

