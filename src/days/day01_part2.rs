pub struct Day1Part2;

impl crate::days::Day for Day1Part2 {
    fn solve(&self, input: &str) -> String {
        input.lines()
            .map(|line| (0..line.len()).map(|i| &line[i..]))
            .map(|line_parts| {
                line_parts.filter_map(|l| {
                        let first_char = l.chars().next().unwrap();
                        if let Some(d) = first_char.to_digit(10) {
                            Some(d)
                        } else if l.starts_with("one") {
                            Some(1)
                        } else if l.starts_with("two") {
                            Some(2)
                        } else if l.starts_with("three") {
                            Some(3)
                        } else if l.starts_with("four") {
                            Some(4)
                        } else if l.starts_with("five") {
                            Some(5)
                        } else if l.starts_with("six") {
                            Some(6)
                        } else if l.starts_with("seven") {
                            Some(7)
                        } else if l.starts_with("eight") {
                            Some(8)
                        } else if l.starts_with("nine") {
                            Some(9)
                        } else {
                            None
                        }
                    })
            })
            .map(|mut ds| {
                let first = ds.next().unwrap();
                first * 10 + ds.next_back().unwrap_or(first)
            })
            .sum::<u32>()
            .to_string()
    }
}
