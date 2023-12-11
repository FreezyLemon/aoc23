pub struct Day11Part1;

impl crate::days::Day for Day11Part1 {
    fn solve(&self, input: String) -> String {
        let cols = 1 + input.find('\n').expect("has LF byte") as i16;
        let rows = input.len() as i16 / cols;

        debug_assert!(
            input.bytes().skip(cols as usize - 1).step_by(cols as usize).all(|b| b == b'\n'),
            "rows differ in length",
        );
        debug_assert_eq!(cols - 1, rows, "input is not square"); // maybe unneeded


        let galaxies: Vec<_> = input.match_indices('#')
            .map(|(idx, _)| (idx as i16 % cols, idx as i16 / cols))
            .collect();

        let empty_cols: Vec<_> = (0..cols - 1)
            .filter(|col| galaxies.iter().all(|(x, _)| x != col))
            .collect();

        let empty_rows: Vec<_> = (0..rows)
            .filter(|row| galaxies.iter().all(|(_, y)| y != row))
            .collect();

        let mut adjusted_galaxies: Vec<_> = galaxies.into_iter()
            .map(|(x, y)| {
                (
                    x + empty_cols.iter().filter(|col| x > **col).count() as i16,
                    y + empty_rows.iter().filter(|row| y > **row).count() as i16,
                )
            })
            .collect();

        let mut sum = 0;

        while let Some((x, y)) = adjusted_galaxies.pop() {
            for (other_x, other_y) in &adjusted_galaxies {
                sum += (other_y - y).abs() as i32 + (other_x - x).abs() as i32;
            }
        }

        sum.to_string()
    }
}

