use crate::LINE_SEPARATOR;

pub struct Day14Part1;

impl crate::days::Day for Day14Part1 {
    fn solve(&self, input: &str) -> String {
        let cols = LINE_SEPARATOR.len() + input.find(LINE_SEPARATOR).expect("has line separator");
        let rows = (input.len() / cols) as i32;

        let mut cubed_rocks: Vec<_> = input.match_indices('#')
            .map(|(idx, _)| ((idx % cols) as i32, (idx / cols) as i32))
            .collect();
        cubed_rocks.extend((0..cols).map(|c| (c as i32, -1)));
        cubed_rocks.sort_unstable();

        let mut round_rocks: Vec<_> = input.match_indices('O')
            .map(|(idx, _)| ((idx % cols) as i32, (idx / cols) as i32))
            .collect();
        round_rocks.sort_unstable();

        cubed_rocks.into_iter()
            .rev()
            .map(|(cubed_col, cubed_row)| {
                let (round_count, col_load) = round_rocks.iter()
                    .filter(|&&(round_col, round_row)| cubed_col == round_col && round_row > cubed_row)
                    .fold((0, 0), |(cnt, acc), _| (cnt + 1, acc + rows - cubed_row - cnt));

                for _ in 0..round_count {
                    round_rocks.pop().unwrap();
                }

                col_load
            })
            .sum::<i32>()
            .to_string()
    }
}
