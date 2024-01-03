pub struct Day1Part1;

impl crate::days::Day for Day1Part1 {
    fn solve(&self, input: &str) -> String {
        input
            .lines()
            .map(str::chars)
            .map(|cs| cs.filter_map(|c| c.to_digit(10)))
            .map(|mut all_digits| {
                let first = all_digits.next().unwrap();
                let last = all_digits.next_back().unwrap_or(first);

                first * 10 + last
            })
            .sum::<u32>()
            .to_string()
    }
}
