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

        let mut start_ids = Vec::new();

        let mut raw_nodes: Vec<RawNode> = lines_iter.skip(1) // empty line
            .map(|line| {
                RawNode {
                    name: &line[0..][..3],
                    left: &line[7..][..3],
                    right: &line[12..][..3],
                }
            })
            .collect();

        raw_nodes.sort_unstable_by_key(|rn| rn.name);
        let nodes: Vec<Node> = raw_nodes.iter()
            .enumerate()
            .map(|(id, raw_node)| {
                let left = raw_nodes.binary_search_by_key(&raw_node.left, |rn| rn.name).unwrap();
                let right = raw_nodes.binary_search_by_key(&raw_node.right, |rn| rn.name).unwrap();

                if raw_node.name.ends_with('A') {
                    start_ids.push(id);
                }
                
                let is_end = raw_node.name.ends_with('Z');

                Node {
                    left: left as u32,
                    right: right as u32,
                    is_end,
                }
            })
            .collect();

        start_ids.into_iter()
            .map(|id| &nodes[id])
            .map(|n| {
                let mut steps = 0;
                let mut curr_node = n;
                for instr in instructions.iter().cycle() {
                    let next_id = match instr {
                        Direction::Left => curr_node.left,
                        Direction::Right => curr_node.right,
                    };

                    if curr_node.is_end {
                        break;
                    }

                    steps += 1;
                    curr_node = &nodes[next_id as usize];
                }

                steps
            })
            .reduce(lcm)
            .unwrap()
            .to_string()
    }
}

fn lcm(a: i64, b: i64) -> i64 {
    (a * b).abs() / gcd(a, b)
}

fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

struct RawNode<'input> {
    name: &'input str,
    left: &'input str,
    right: &'input str,
}

#[derive(Debug)]
struct Node {
    left: u32,
    right: u32,
    is_end: bool,
}

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

