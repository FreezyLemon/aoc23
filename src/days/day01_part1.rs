pub struct Day1Part1;

impl crate::days::Day for Day1Part1 {
    fn solve(&self, input: &str) -> String {
        let calibration_value: u32 = input
            .lines()
            .map(|l| {
                let mut all_digits = l.chars().filter_map(|c| c.to_digit(10));
                let first = all_digits.next().unwrap();
                let last = all_digits.next_back().unwrap_or(first);

                first * 10 + last
            })
            .sum();

        calibration_value.to_string()
    }
}

