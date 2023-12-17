pub struct Day12Part1;

impl crate::days::Day for Day12Part1 {
    fn solve(&self, input: &str) -> String {
        input.lines()
            .map(|line| {
                let (pattern, arrangement) = line.split_once(' ').expect("input has whitespace");
                let arrangement: Vec<i32> = arrangement.split(',')
                    .map(str::parse)
                    .collect::<Result<_, _>>()
                    .expect("arrangement can be parsed to int");

                (pattern, arrangement)
            })
            .map(|(pat, arrng)| {
                let pat = pat.trim_matches('.').as_bytes();
                munch(pat, &arrng)
            })
            .sum::<i32>()
            .to_string()
    }
}

fn munch(pattern: &[u8], arrangements: &[i32]) -> i32 {
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

    let next_arrng = arrangements[0] as usize;
    let max_offset = match pattern.iter().enumerate().find(|(_, b)| **b == b'#') {
        Some((idx, _)) => std::cmp::min(max_offset, idx as i32),
        None => max_offset,
    };

    let mut sum = 0;
    for offset in 0..=max_offset as usize {
        let offset_pat = &pattern[offset..];
        if can_be_contiguous(&offset_pat[..next_arrng]) {
            match offset_pat[next_arrng..].first() {
                None if arrangements.len() == 1 => sum += 1, // end of pattern
                Some(b'#') => continue, // arrangements cannot be divided by a #
                Some(_) => sum += munch(&offset_pat[1 + next_arrng..], &arrangements[1..]),
                _ => {}
            }
        }
    }

    sum
}

fn can_be_contiguous(s: &[u8]) -> bool {
    !s.contains(&b'.')
}
