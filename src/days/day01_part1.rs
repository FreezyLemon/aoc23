pub struct Day1Part1;

impl crate::days::Day for Day1Part1 {
    fn solve(&self, input: String) -> String {
        let calibration_value: u32 = input
            .lines()
            .map(|l| {
                let all_digits: Vec<u32> = l.chars()
                    .filter_map(|c| c.to_digit(10))
                    .collect();

                all_digits[0] * 10 + all_digits.last().unwrap()
            })
            .sum();

        calibration_value.to_string()
    }
}

