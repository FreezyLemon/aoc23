pub struct Day6Part2;

impl crate::days::Day for Day6Part2 {
    fn solve(&self, input: &str) -> String {
        let (time, distance) = input.split_once('\n').expect("has 1+ newline");

        let (_, time) = time.split_once(':').expect("has :");
        let (_, distance) = distance.split_once(':').expect("has :");

        // assume all entries are valid ints
        let time: u64 = time.split_ascii_whitespace()
            .flat_map(str::chars)
            .collect::<String>()
            .parse()
            .expect("is valid int");

        let record: u64 = distance.split_ascii_whitespace()
            .flat_map(str::chars)
            .collect::<String>()
            .parse()
            .expect("is valid int");

        let mut upper_bound = time / 2;
        let mut lower_bound = 1;
        let mut idx = (upper_bound + lower_bound) / 2;
        while upper_bound >= lower_bound {
            idx = (upper_bound + lower_bound) / 2;
            let next_idx = idx + 1;

            if idx * (time - idx) > record {
                upper_bound = idx - 1;
            } else if next_idx * (time - next_idx) <= record {
                lower_bound = idx + 1;
            } else {
                break;
            }
        }

        (time - 1 - 2 * idx).to_string()
    }
}
