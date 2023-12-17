pub struct Day1Part2Malox;

impl crate::days::Day for Day1Part2Malox {
    fn solve(&self, input: &str) -> String {
        let mut number_digit_map = vec![
            (String::from("one"), 1),
            (String::from("two"), 2),
            (String::from("three"), 3),
            (String::from("four"), 4),
            (String::from("five"), 5),
            (String::from("six"), 6),
            (String::from("seven"), 7),
            (String::from("eight"), 8),
            (String::from("nine"), 9),
        ];

        number_digit_map.extend((1..=9).map(|d| (d.to_string(), d)));

        let result: u32 = input.lines()
            .map(|line| {
                let (_, first_val) = number_digit_map
                    .iter()
                    .map(|(k, v)| (line.find(k), v))
                    .filter(|(k, _)| k.is_some())
                    .min_by_key(|(k, _)| k.unwrap())
                    .unwrap();

                let (_, last_val) = number_digit_map
                    .iter()
                    .map(|(k, v)| (line.rfind(k), v))
                    .filter(|(k, _)| k.is_some())
                    .max_by_key(|(k, _)| k.unwrap())
                    .unwrap();

                first_val * 10 + last_val
            })
            .sum();

        result.to_string()
    }
}

