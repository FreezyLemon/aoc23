use aoc23::input::get_input;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let calibration_value: u32 = get_input()?
        .split('\n')
        .map(|l| {
            let all_digits: Vec<u32> = l.chars()
                .filter_map(|c| c.to_digit(10))
                .collect();

            all_digits[0] * 10 + all_digits.last().unwrap()
        })
        .sum();

    println!("calibration value is {calibration_value}");
    Ok(())
}

