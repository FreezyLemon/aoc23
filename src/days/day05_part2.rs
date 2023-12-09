use std::cmp::{max, min};

pub struct Day5Part2;

impl crate::days::Day for Day5Part2 {
    fn solve(&self, input: String) -> String {
        let mut chunks_iter = input.split("\n\n");

        let (_, seed_range_str) = chunks_iter.next().expect("has content").split_once(':').expect("has :");

        let ints: Vec<i64> = seed_range_str.trim()
            .split_ascii_whitespace()
            .map(str::parse)
            .map(Result::unwrap)
            .collect();
        let mut seed_ranges: Vec<_> = ints.chunks_exact(2)
            .map(|c| (c[0]..c[0] + c[1], 0))
            .collect();

        let map_categories = chunks_iter.map(|c| c.lines().skip(1))
            .map(|lines| {
                lines.map(str::split_ascii_whitespace)
                     .map(|words| {
                        let mut ints = words.map(str::parse::<i64>).map(Result::unwrap);
                        let dst_start = ints.next().unwrap();
                        let src_start = ints.next().unwrap();
                        let len = ints.next().unwrap();

                        (src_start..=src_start - 1 + len, dst_start-src_start)
                     })
                     .collect::<Vec<_>>()
            });

        for category in map_categories {
            for (map_range, offset) in category {
                // println!("applying ({map_range:?}, {offset}) to {seed_ranges:?}");

                let map_s = *map_range.start();
                let map_e = *map_range.end() + 1;

                let mut next_ranges = Vec::new();
                for (seed_range, val) in seed_ranges {
                    let seed_s = seed_range.start;
                    let seed_e = seed_range.end;

                    // check for intersection
                    let lower_bound = max(map_s, seed_s);
                    let upper_bound = min(map_e, seed_e);

                    if lower_bound < upper_bound {
                        next_ranges.push((lower_bound..upper_bound, val + offset));

                        // leftovers
                        if seed_s < lower_bound {
                            next_ranges.push((seed_s..lower_bound, val));
                        }

                        if upper_bound < seed_e {
                            next_ranges.push((upper_bound..seed_e, val));
                        }
                    } else {
                        next_ranges.push((seed_s..seed_e, val));
                    }
                }

                seed_ranges = next_ranges;
                // println!("resulting in {seed_ranges:?}");
                // println!();
            }

            // println!();
        }

        // println!("{seed_ranges:?}");

        let final_vals: Vec<i64> = seed_ranges.into_iter()
            .map(|(range, offset)| range.start + offset)
            .collect();

        println!("{final_vals:?}");

        final_vals.into_iter()
            .min()
            .unwrap()
            .to_string()
    }
}



