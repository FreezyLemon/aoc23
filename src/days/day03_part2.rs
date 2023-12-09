pub struct Day3Part2;

// assumption: all lines are the same length
// (the input is a square)

impl crate::days::Day for Day3Part2 {
    fn solve(&self, input: String) -> String {
        let gear_candidates: Vec<(usize, usize)> = input.lines()
            .map(str::chars)
            .map(Iterator::enumerate)
            .enumerate()
            .flat_map(|(y, chars)| {
                chars.filter(|(_, c)| c == &'*')
                    .map(move |(x, _)| (x, y))
            })
            .collect();

        let part_nos: Vec<_> = input.lines()
            .enumerate()
            .flat_map(|(y, l)| {
                let mut part_nos_with_position = Vec::new();

                let mut s = String::new();
                for (x, c) in l.chars().enumerate() {
                    if c.is_ascii_digit() {
                        s.push(c);
                    } else if !s.is_empty() {
                        let part_no: i32 = s.parse().unwrap();
                        part_nos_with_position.push((
                            part_no,
                            ((x - 1).saturating_sub(s.chars().count())..=x),
                            (y.saturating_sub(1)..=y.saturating_add(1)),
                        ));
                        s.clear();
                    }
                }

                if !s.is_empty() {
                    let x = l.chars().count();
                    let part_no: i32 = s.parse().unwrap();
                    part_nos_with_position.push((
                        part_no,
                        ((x - 1).saturating_sub(s.chars().count())..=x),
                        (y.saturating_sub(1)..=y.saturating_add(1)),
                    ));
                }

                part_nos_with_position
            })
            .collect();

        // println!("{part_nos:#?}");

        let mut gear_ratio_sum = 0;
        for (x, y) in gear_candidates {
            let adjacent_nos: Vec<i32> = part_nos.iter()
                .filter(|(_, xr, yr)| xr.contains(&x) && yr.contains(&y))
                .map(|(pno, _, _)| *pno)
                .collect();

            if adjacent_nos.len() >= 2 {
                gear_ratio_sum += adjacent_nos
                    .into_iter()
                    .reduce(|acc, x| acc * x)
                    .unwrap();
            }
        }

        gear_ratio_sum.to_string()
    }
}

