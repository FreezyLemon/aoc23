use crate::DOUBLE_LINE_SEPARATOR;

pub struct Day13Part1;

// assumption: all cols (rows) inside a pattern are of the same length
impl crate::days::Day for Day13Part1 {
    fn solve(&self, input: &str) -> String {
        let patterns: Vec<_> = input.split(DOUBLE_LINE_SEPARATOR)
            .map(|pat| {
                let lines: Vec<&str> = pat.lines().collect();
                let col_len = lines.len();
                let row_len = lines[0].len();

                let mut line_chars: Vec<_> = lines.iter()
                    .copied()
                    .map(str::chars)
                    .collect();

                let mut pat_vert = vec![0u32; lines[0].len()];
                for col in &mut pat_vert.iter_mut() {
                    for (y, chars) in line_chars.iter_mut().enumerate() {
                        if chars.next().expect("valid input") == '#' {
                            *col |= 1 << y;
                        }
                    }
                }

                let pat_horz: Vec<u32> = lines.into_iter()
                    .map(str::char_indices)
                    .map(|char_indices| char_indices
                        .fold(0_u32, |acc, (x, char)| {
                            if char == '#' {
                                acc | (1 << x)
                            } else {
                                acc
                            }
                        })
                    )
                    .collect();

                ((row_len, pat_horz), (col_len, pat_vert))
            })
            .collect();
    
        patterns.into_iter()
            .map(|((h_len, horz), (v_len, vert))| {
                if let Some(col_no) = find_reflection(&horz, h_len) {
                    col_no
                } else if let Some(row_no) = find_reflection(&vert, v_len) {
                    100 * row_no
                } else {
                    panic!("can't find reflection for a pattern");
                }
            })
            .sum::<usize>()
            .to_string()
    }
}

fn find_reflection(pattern: &[u32], pat_len: usize) -> Option<usize> {
    (1..pat_len)
        .find(|&candidate_idx| {
            pattern.iter()
                .all(|row_or_col| {
                    let bit_len = std::cmp::min(candidate_idx, pat_len - candidate_idx);
                    let mask = 0xFFFF_FFFF >> (32 - bit_len);

                    let other = mask & (row_or_col << (32 - candidate_idx)).reverse_bits();
                    let rest = mask & (row_or_col >> candidate_idx);

                    rest == other
                })
        })
}
