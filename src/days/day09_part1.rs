pub struct Day9Part1;

impl crate::days::Day for Day9Part1 {
    fn solve(&self, input: &str) -> String {
        input.lines()
            .map(str::split_ascii_whitespace)
            .map(|split| {
                let first_history: Vec<i32> = split
                    .map(str::parse)
                    .map(Result::unwrap)
                    .collect();

                let mut all_histories = Vec::with_capacity(first_history.len());
                all_histories.push(first_history);
                while let Some(history) = all_histories.last() {
                    if history.iter().all(|&i| i == 0) {
                        break;
                    }

                    let new_history: Vec<i32> = history.windows(2)
                        .map(|window| window[1] - window[0])
                        .collect();

                    all_histories.push(new_history);
                }

                all_histories
            })
            .map(|histories| {
                histories.into_iter()
                    .map(|h| *h.last().unwrap())
                    .sum::<i32>()
            })
            .sum::<i32>()
            .to_string()
    }
}

