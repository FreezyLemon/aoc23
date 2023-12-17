pub struct Day3Part1;

// assumption: all lines are the same length
// (the input is a square)

impl crate::days::Day for Day3Part1 {
    fn solve(&self, input: &str) -> String {
        let lines: Vec<_> = input.lines().collect();
        
        let symbol_idxs: Vec<(i32, i32)> = lines.iter()
            .map(|l| l
                 .match_indices(char_is_symbol)
                 .map(|(idx, _)| idx)
            )
            .enumerate()
            .flat_map(|(y, idxs)| idxs.map(move |x| (x as i32, y as i32)))
            .collect();

        let mut part_numbers: Vec<u32> = Vec::new();

        for (y, line) in lines.into_iter().enumerate() {
            let mut cur_number_string = String::with_capacity(8);
            let mut cur_number_is_adjacent = false;
            for (x, c) in line.chars().enumerate() {
                if c.is_ascii_digit() {
                    cur_number_string.push(c);

                    if !cur_number_is_adjacent &&
                        symbol_idxs
                            .iter()
                            .any(|(sx, sy)|
                                 (x as i32 - 1..=x as i32 + 1).contains(sx) &&
                                 (y as i32 - 1..=y as i32 + 1).contains(sy)
                            ) {
                        cur_number_is_adjacent = true;
                    }
                } else {
                    if cur_number_is_adjacent && !cur_number_string.is_empty() {
                        part_numbers.push(cur_number_string.parse().expect("can be parsed as number"));
                    }
                    cur_number_string.clear();
                    cur_number_is_adjacent = false;
                }
            }

            // check once at end of line, too
            if cur_number_is_adjacent && !cur_number_string.is_empty() {
                part_numbers.push(cur_number_string.parse().expect("can be parsed"));
            }
        }

        part_numbers.into_iter().sum::<u32>().to_string()
    }
}

fn char_is_symbol(c: char) -> bool {
    !c.is_ascii_digit() && c != '.'
}

