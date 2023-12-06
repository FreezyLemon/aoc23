// use std::collections::VecDeque;

// use std::cmp::max;

// pub struct Day3Part2;

// // assumption: all lines are the same length
// // (the input is a square)

// impl crate::days::Day for Day3Part2 {
//     fn solve(&self, input: String) -> String {
//         let lines: Vec<_> = input.lines().collect();
        
//         let mut last_line_part_nos: VecDeque<((usize, usize), u32)> = VecDeque::new();
//         let mut gears_with_one_part_no: VecDeque<(usize, u32)> = VecDeque::new();

//         for line in &lines {
//             let mut running_part_no = None;

//             for (x, c) in line.chars().enumerate() {
//                 if c.is_ascii_digit() {
//                     let curr_x = max(0, x); // TODO: Fix this mess

//                     running_part_no = match running_part_no {
//                         None => Some(((curr_x, curr_x), c.to_digit(10).unwrap())),
//                         Some(((x_start, x_end), curr_val)) => Some((
//                             (x_start, curr_x),
//                             curr_val * 10 + c.to_digit(10).unwrap()
//                         )),
//                     };
//                 } else {
//                     if c == '*' && running_part_no.is_some() {
//                         gears_with_one_part_no.push_back((x, running_part_no.unwrap().1));
//                     }

//                     if let Some(part_no) = running_part_no.take() {
//                         last_line_part_nos.push_back(part_no);
//                     }
//                 }
//             }
//         }


//         let gear_idxs: Vec<(i32, i32)> = lines.iter()
//             .map(|l| l
//                  .match_indices(|c| c == '*')
//                  .map(|(idx, _)| idx)
//             )
//             .enumerate()
//             .flat_map(|(y, idxs)| idxs.map(move |x| (x as i32, y as i32)))
//             .collect();

//         // println!("{symbol_idxs:#?}");

//         'gear: for (gx, gy) in gear_idxs {
//             let mut adj_number = false;
//             for x in gx-1..gx+1 {
//                 for y in gy-1..gx+1 {
//                     if lines.get(y as usize).map_or(false, |l| l.chars().nth(x as usize).map_or(false, |c| c.is_ascii_digit())) {
//                         if adj_number {
//                             // oh shit
//                             continue 'gear;
//                         } else {
//                             adj_number = true;
//                         }
//                     }
//                 }
//             }
//         }
        
//         "".into()
//     }
// }

// fn char_is_symbol(c: char) -> bool {
//     !c.is_ascii_digit() && c != '.'
// }

