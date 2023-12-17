use crate::LINE_SEPARATOR;

pub struct Day11Part2;

impl crate::days::Day for Day11Part2 {
    fn solve(&self, input: &str) -> String {
        let cols = (LINE_SEPARATOR.len() + input.find(LINE_SEPARATOR).expect("has LF byte")) as i32;
        let rows = input.len() as i32 / cols;

        debug_assert!(
            input.bytes().skip(cols as usize - 1).step_by(cols as usize).all(|b| b == b'\n'),
            "rows differ in length",
        );
        debug_assert_eq!(cols - LINE_SEPARATOR.len() as i32, rows, "input is not square"); // maybe unneeded


        let galaxies: Vec<_> = input.match_indices('#')
            .map(|(idx, _)| (idx as i32 % cols, idx as i32 / cols))
            .collect();

        let empty_cols: Vec<_> = (0..cols - 1)
            .filter(|col| galaxies.iter().all(|(x, _)| x != col))
            .collect();

        let empty_rows: Vec<_> = (0..rows)
            .filter(|row| galaxies.iter().all(|(_, y)| y != row))
            .collect();

        let mut adjusted_galaxies: Vec<_> = galaxies.into_iter()
            .map(|(x, y)| {
                let col_count = empty_cols.iter().filter(|col| x > **col).count() as i32;
                let row_count = empty_rows.iter().filter(|row| y > **row).count() as i32;
                let multiplier = 1_000_000;

                (
                    x + (multiplier - 1) * col_count,
                    y + (multiplier - 1) * row_count,
                )
            })
            .collect();

        let mut sum = 0;

        while let Some((x, y)) = adjusted_galaxies.pop() {
            for (other_x, other_y) in &adjusted_galaxies {
                sum += (other_y - y).abs() as i64 + (other_x - x).abs() as i64;
            }
        }

        sum.to_string()
    }
}

