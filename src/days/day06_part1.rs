pub struct Day6Part1;

impl crate::days::Day for Day6Part1 {
    fn solve(&self, input: String) -> String {
        let (times, distances) = input.split_once('\n').expect("has 1+ newline");

        let (_, times) = times.split_once(':').expect("has :");
        let (_, distances) = distances.split_once(':').expect("has :");

        // assume all entries are valid ints
        let times = times.split_ascii_whitespace().map(str::parse).map(Result::unwrap);
        let records = distances.split_ascii_whitespace().map(str::parse).map(Result::unwrap);
        
        times
            .zip(records)
            .map(|(t, r)| {
                (1..t).zip((1..t).rev()) // possible accel/move combinations
                    .map(|(x, y)| x * y) // resulting distances
                    .filter(|d| *d > r)
                    .count()
            })
            .reduce(|acc, x| acc * x)
            .expect("is not empty")
            .to_string()
    }
}
