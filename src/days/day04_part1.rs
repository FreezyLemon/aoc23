pub struct Day4Part1;

impl crate::days::Day for Day4Part1 {
    fn solve(&self, input: String) -> String {
        input.lines()
            .map(|l| l.split_once(':').expect("input contains :"))
            .map(|(_card, rest)| rest.split_once('|').expect("input contains |"))
            .map(|(left, right)| {
                let winners: Vec<u32> = left.split_ascii_whitespace()
                    .map(|w| w.parse().expect("is integer"))
                    .collect();

                let win_amount = right.split_ascii_whitespace()
                    .map(|w| w.parse().expect("is integer"))
                    .map(|i| winners.contains(&i))
                    .filter(|b| *b)
                    .count() as u32;

                if win_amount > 0 {
                    2_u32.pow(win_amount - 1)
                } else {
                    0
                }
            })
            .sum::<u32>()
            .to_string()
    }
}
