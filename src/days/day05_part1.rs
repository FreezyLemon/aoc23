pub struct Day5Part1;

impl crate::days::Day for Day5Part1 {
    fn solve(&self, input: String) -> String {
        let mut chunks_iter = input.split("\n\n");

        let mut seeds: Vec<i64> = chunks_iter.next().expect("has content")
            .split_once(':').expect("has :")
            .1.split_ascii_whitespace()
            .map(|w| w.parse().expect("is integer"))
            .collect();

        let map_categories = chunks_iter.map(|c| c.lines().skip(1))
            .map(|lines| {
                lines.map(str::split_ascii_whitespace)
                     .map(|words| {
                        let mut ints = words.map(str::parse::<i64>).map(Result::unwrap);
                        let dst_start = ints.next().unwrap();
                        let src_start = ints.next().unwrap();
                        let len = ints.next().unwrap();

                        (src_start..src_start + len, dst_start-src_start)
                     })
                     .collect::<Vec<_>>()
            });

        for maps in map_categories {
            'seed: for s in &mut seeds {
                for (range, offset) in &maps {
                    if range.contains(s) {
                        *s += offset;
                        continue 'seed;
                    }
                }
            }
        }

        seeds.into_iter().min().expect("has at least one value").to_string()
    }
}
