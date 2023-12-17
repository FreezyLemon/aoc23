pub struct Day2Part1;

impl crate::days::Day for Day2Part1 {
    fn solve(&self, input: &str) -> String { 
        input.lines()
            .enumerate()
            .filter(|(_, l)| {
                l.split_once(": ")
                    .expect("line has :")
                    .1
                    .split("; ")
                    .all(valid_cubeset)
            })
            .map(|(idx, _)| 1 + idx as u32)
            .sum::<u32>()
            .to_string()
    }
}

fn valid_cubeset(s: &str) -> bool {
    s.split(", ")
        .all(|color| {
            let (amount, color) = color.split_once(' ').unwrap();
            let Ok(amount) = amount.parse::<u8>() else {
                return false;
            };

            match color {
                "red" if amount <= 12 => true, 
                "green" if amount <= 13 => true,
                "blue" if amount <= 14 => true,
                _ => false
            }
        })
}

