use std::collections::HashMap;

pub struct Day12Part2;

impl crate::days::Day for Day12Part2 {
    fn solve(&self, input: String) -> String {
        input.lines()
            .map(|line| {
                let (pattern, arrangement) = line.split_once(' ').expect("input has whitespace");
                let pattern = (0..5)
                    .map(|_| pattern.to_string())
                    .collect::<Vec<_>>()
                    .join("?");
                
                let arrangement: Vec<i32> = arrangement.split(',')
                    .map(str::parse)
                    .collect::<Result<_, _>>()
                    .expect("arrangement can be parsed to int");

                let arrangement: Vec<i32> = (0..5).flat_map(|_| &arrangement)
                    .copied()
                    .collect();

                (pattern, arrangement)
            })
            .map(|(pat, arrng)| {
                let mut cache = HashMap::new();
                let pat = pat.trim_matches('.').as_bytes();
                munch(pat, &arrng, pat.as_ptr(), arrng.as_ptr(), &mut cache)
            })
            .sum::<i64>()
            .to_string()
    }
}

fn munch(pattern: &[u8], arrangements: &[i32], orig_pat: *const u8, orig_arrng: *const i32, cache: &mut HashMap<(isize, isize), i64>) -> i64 {
    if arrangements.is_empty() {
        if pattern.contains(&b'#') {
            // there are still unused good springs,
            // so we can't finish the pattern here
            return 0;
        } else {
            return 1;
        }
    }

    let needed_len = -1 + arrangements.len() as i32 + arrangements.iter().sum::<i32>();
    let max_offset = pattern.len() as i32 - needed_len;
    if max_offset < 0 {
        return 0;
    }

    let next_arrng_offset = arrangements[0] as usize;
    let max_offset = match pattern.iter().enumerate().find(|(_, b)| **b == b'#') {
        Some((idx, _)) => std::cmp::min(max_offset, idx as i32),
        None => max_offset,
    };

    let mut sum = 0;
    for offset in 0..=max_offset as usize {
        let offset_pat = &pattern[offset..];
        if can_be_contiguous(&offset_pat[..next_arrng_offset]) {
            match offset_pat[next_arrng_offset..].get(0) {
                None if arrangements.len() == 1 => sum += 1, // end of pattern
                Some(b'#') => continue, // arrangements cannot be divided by a #
                Some(_) => {
                    let next_pat = &offset_pat[1 + next_arrng_offset..];
                    let next_arrng = &arrangements[1..];
                    let pat_offset = unsafe { next_pat.as_ptr().offset_from(orig_pat) };
                    let arrng_offset = unsafe { next_arrng.as_ptr().offset_from(orig_arrng) };

                    if let Some(v) = cache.get(&(pat_offset, arrng_offset)) {
                        sum += v;
                    } else {
                        let new_val = munch(next_pat, next_arrng, orig_pat, orig_arrng, cache);
                        sum += new_val;
                        cache.insert((pat_offset, arrng_offset), new_val);
                    }
                },
                _ => {}
            }
        }
    }

    sum
}

fn can_be_contiguous(s: &[u8]) -> bool {
    !s.contains(&b'.')
}
