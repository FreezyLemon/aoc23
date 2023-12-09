use std::io::Write;

pub struct Day8Part2;

impl crate::days::Day for Day8Part2 {
    fn solve(&self, input: String) -> String {
        let mut lines_iter = input.lines();
        let instructions = lines_iter.next().expect("has at least one line")
            .chars()
            .map(|c| match c {
                'R' => Direction::Right,
                'L' => Direction::Left,
                _ => panic!("invalid input"),
            });
 
        lines_iter.next().unwrap(); // ignore empty line
 
        let raw_nodes: Vec<(String, RawNode)> = lines_iter.map(|l| l.split_once('=').expect("line has ="))
            .map(|(name, paths)| {
                let (l, r) = paths.trim().split_once(',').expect("line has ,");

                (
                    name.trim().to_string(),
                    RawNode {
                        left: l.trim_start_matches('(').to_string(),
                        right: r.trim().trim_end_matches(')').to_string(),
                    }
                )
            })
            .collect();

        let start_node_ids: Vec<usize> = raw_nodes.iter()
            .enumerate()
            .filter(|(_, (name, _))| name.ends_with('A'))
            .map(|(idx, _)| idx)
            .collect();

        let end_node_ids: Vec<usize> = raw_nodes.iter()
            .enumerate()
            .filter(|(_, (name, _))| name.ends_with('Z'))
            .map(|(idx, _)| idx)
            .collect();

        let parsed_nodes: Vec<Node> = raw_nodes.iter()
            .enumerate()
            .map(|(idx, (_, node))| {
                let (left_idx, _) = raw_nodes.iter()
                    .enumerate()
                    .find(|(_, (name, _))| name == &node.left)
                    .unwrap();

                let (right_idx, _) = raw_nodes.iter()
                    .enumerate()
                    .find(|(_, (name, _))| name == &node.right)
                    .unwrap();

                Node {
                    id: u16::try_from(idx).unwrap(),
                    left: u16::try_from(left_idx).unwrap(),
                    right: u16::try_from(right_idx).unwrap(),
                }
            })
            .collect();

        drop(raw_nodes);

        let mut node_buffer: Vec<&Node> = start_node_ids.into_iter()
            .map(|id| &parsed_nodes[id])
            .collect();

        let mut stdout = std::io::stdout().lock();
        let mut steps: i64 = 0;
        for instr in instructions.cycle() {
            if steps % 10_000_000 == 0 {
                stdout.write_all(format!("\rfinished step {}M", steps / 1_000_000).as_bytes()).unwrap();
                stdout.flush().unwrap();
            }
            for node in &mut node_buffer {
                let next_node_id = match instr {
                    Direction::Left => node.left,
                    Direction::Right => node.right,
                };

                *node = &parsed_nodes[next_node_id as usize];
            }

            steps += 1;

            if node_buffer.iter().all(|n| end_node_ids.contains(&(n.id as usize))) {
                break;
            }
        }

        // reset stdout to overwrite the "finished step X"
        stdout.write(b"\r").unwrap();
        drop(stdout); // this also flushes IIRC

        steps.to_string()
    }
}

#[derive(Debug, Clone)]
struct RawNode {
    left: String,
    right: String,
}

#[derive(Debug)]
struct Node {
    id: u16,
    left: u16,
    right: u16,
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Left,
    Right,
}

