// 
// fn main() {
//     let file_content = std::fs::read_to_string("input.txt").unwrap();
//     let calibration_value: u32 = file_content
//         .split('\n')
//         .map(|l| {
//             let all_digits: Vec<u32> = l.chars()
//                 .filter_map(|c| c.to_digit(10))
//                 .collect();
// 
//             all_digits[0] * 10 + all_digits.last().unwrap()
//         })
//         .sum();
// 
//     println!("calibration value is {calibration_value}");
// }
// 

fn main() {
    let file_content = std::fs::read_to_string("input.txt").unwrap();

    let mut numbers = Vec::new();
    for line in file_content.split('\n') {
        let mut v = Vec::new();
        
        let mut i = 0;
        while i < line.len() {
            let l = &line[i..];
            let first_char = l.chars().next().unwrap();
            if let Some(d) = first_char.to_digit(10) {
                v.push(d);
                i += 1;
                continue;
            }

            if l.starts_with("one") {
                v.push(1);
            } else if l.starts_with("two") {
                v.push(2);
            } else if l.starts_with("three") {
                v.push(3);
            } else if l.starts_with("four") {
                v.push(4);
            } else if l.starts_with("five") {
                v.push(5);
            } else if l.starts_with("six") {
                v.push(6);
            } else if l.starts_with("seven") {
                v.push(7);
            } else if l.starts_with("eight") {
                v.push(8);
            } else if l.starts_with("nine") {
                v.push(9);
            }

            i += 1;
        }

        numbers.push(v);
    }

    let result: u32 = numbers.into_iter()
        .map(|ds| ds.first().unwrap().clone() * 10 + ds.last().unwrap().clone())
        .sum();

    println!("calibration value is {result}");
}
