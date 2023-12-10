pub struct Day4Part2;

impl crate::days::Day for Day4Part2 {
    fn solve(&self, input: String) -> String {
        let card_count = input.lines().count();

        input.lines()
            .map(|l| l.split_once(':').expect("input contains :"))
            .map(|(_card, rest)| rest.split_once('|').expect("input contains |"))
            .map(|(left, right)| {
                let winners: Vec<u32> = left.split_ascii_whitespace()
                    .map(|w| w.parse().expect("is integer"))
                    .collect();

                right.split_ascii_whitespace()
                    .map(|w| w.parse().expect("is integer"))
                    .map(|i| winners.contains(&i))
                    .filter(|b| *b)
                    .count() as u32
            })
            .enumerate()
            .filter(|(_, w)| *w > 0)
            .fold(vec![1u32; card_count], |mut amnts: Vec<u32>, (idx, win_amount)| {
                let curr_amount = amnts[idx];
                for amnt in amnts.iter_mut().skip(idx + 1).take(win_amount as usize) {
                    *amnt += curr_amount;
                }
                amnts
            })
            .into_iter()
            .sum::<u32>()
            .to_string()
    }
}
