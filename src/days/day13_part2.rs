use crate::DOUBLE_LINE_SEPARATOR;

pub struct Day13Part2;

// assumption: all cols (rows) inside a pattern are of the same length
impl crate::days::Day for Day13Part2 {
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
                        .filter(|(_, char)| *char == '#')
                        .fold(0_u32, |acc, (x, _)| acc | (1 << x))
                    )
                    .collect();

                ((row_len, pat_horz), (col_len, pat_vert))
            })
            .collect();
    
        patterns.into_iter()
            .map(|((h_len, horz), (v_len, vert))| {
                let base_reflection = if let Some(col_no) = find_reflection(&horz, h_len, None) {
                    Reflection::Column(col_no)
                } else if let Some(row_no) = find_reflection(&vert, v_len, None) {
                    Reflection::Row(row_no)
                } else {
                    panic!("can't find reflection for a pattern");
                };

                (base_reflection, (h_len, horz), (v_len, vert))
            })
            .map(|(old_reflection, (h_len, horz), (v_len, vert))| {
                let mut new_reflection = None;
                let mut horz_old = None;
                let mut vert_old = None;

                match old_reflection {
                    Reflection::Column(c) => horz_old = Some(c),
                    Reflection::Row(r) => vert_old = Some(r),
                }
                for i in 0..horz.len() {
                    for j in 0..h_len {
                        let mut horz = horz.clone();
                        horz[i] ^= 1 << j;
                        if let Some(col_no) = find_reflection(&horz, h_len, horz_old) {
                            new_reflection = Some(Reflection::Column(col_no));
                        }
                    }
                }

                for i in 0..vert.len() {
                    for j in 0..v_len {
                        let mut vert = vert.clone();
                        vert[i] ^= 1 << j;
                        if let Some(row_no) = find_reflection(&vert, v_len, vert_old) {
                            new_reflection = Some(Reflection::Row(row_no));
                        }
                    }
                }

                new_reflection.expect("could find new reflection").get_index()
            })
            .sum::<usize>()
            .to_string()
    }
}

#[derive(Debug, Clone, Copy)]
enum Reflection {
    Column(usize),
    Row(usize),
}

impl Reflection {
    fn get_index(self) -> usize {
        match self {
            Reflection::Column(c) => c,
            Reflection::Row(r) => 100 * r,
        }
    }
}

fn find_reflection(pattern: &[u32], pat_len: usize, old_reflection: Option<usize>) -> Option<usize> {
    (1..pat_len)
        .filter(|idx| old_reflection.map_or(true, |r| r != *idx))
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
