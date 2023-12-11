use std::{cmp::{max, min}, ops::RangeInclusive};

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
            .map(|c| (c[0]..=c[0] - 1 + c[1], 0))
            .collect();

        let map_categories = chunks_iter.map(|c| c.lines().skip(1))
            .map(|lines| {
                lines.map(str::split_ascii_whitespace)
                     .map(|words| {
                        let mut ints = words.map(str::parse::<i64>).map(Result::unwrap);
                        let dst_start = ints.next().unwrap();
                        let src_start = ints.next().unwrap();
                        let len = ints.next().unwrap();

                        debug_assert!(len > 0);

                        (src_start..=src_start - 1 + len, dst_start-src_start)
                     })
                     .collect::<Vec<_>>()
            });

        for category in map_categories {
            let mut next_ranges = Vec::new();
            println!("{category:?}");
            println!();
            for (map_range, offset) in category {
                let mut unmapped_ranges = Vec::new();
                for (seed_range, val) in seed_ranges {
                    let (unmapped, mapped) = apply_map(seed_range, map_range.clone(), val, offset);
                    next_ranges.extend(mapped);
                    unmapped_ranges.extend(unmapped);
                }

                seed_ranges = unmapped_ranges;
            }

            seed_ranges.extend(next_ranges);
        }

        let final_vals: Vec<i64> = seed_ranges.into_iter()
            .map(|(range, offset)| range.start() + offset)
            .collect();

        final_vals.into_iter()
            .min()
            .unwrap()
            .to_string()
    }
}

fn apply_map(range: RangeInclusive<i64>, map: RangeInclusive<i64>, val: i64, map_offset: i64) -> (Vec<(RangeInclusive<i64>, i64)>, Vec<(RangeInclusive<i64>, i64)>) {
    let mut unmapped = Vec::new();
    let mut mapped = Vec::new();

    let lower_bound = *max(range.start(), map.start());
    let upper_bound = *min(range.end(), map.end());

    if upper_bound >= lower_bound {
        mapped.push((lower_bound..=upper_bound, val + map_offset));

        if *range.start() < lower_bound {
            unmapped.push((*range.start()..=lower_bound - 1, val));
        }

        if upper_bound < *range.end() {
            unmapped.push((upper_bound + 1..=*range.end(), val));
        }
    } else {
        unmapped.push((range, val));
    }

    (unmapped, mapped)
}

