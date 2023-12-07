pub struct Day6Part2;

impl crate::days::Day for Day6Part2 {
    fn solve(&self, input: String) -> String {
        let (time, distance) = input.split_once('\n').expect("has 1+ newline");

        let (_, time) = time.split_once(':').expect("has :");
        let (_, distance) = distance.split_once(':').expect("has :");

        // assume all entries are valid ints
        let time: i64 = time.split_ascii_whitespace()
            .flat_map(str::chars)
            .collect::<String>()
            .parse()
            .expect("is valid int");

        let record_dist: i64 = distance.split_ascii_whitespace()
            .flat_map(str::chars)
            .collect::<String>()
            .parse()
            .expect("is valid int");
        
        (1..time).zip((1..time).rev()) // possible accel/move combinations
            .map(|(x, y)| x * y) // resulting distances
            .filter(|d| *d > record_dist)
            .count()
            .to_string()
    }
}
