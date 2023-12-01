fn main() {
    let file_content = std::fs::read_to_string("input.txt").unwrap();
    let calibration_value: u32 = file_content
        .split('\n')
        .map(|l| {
            let all_digits: Vec<u32> = l.chars()
                .filter_map(|c| c.to_digit(10))
                .collect();

            all_digits[0] * 10 + all_digits.last().unwrap()
        })
        .sum();

    println!("calibration value is {calibration_value}");
}
