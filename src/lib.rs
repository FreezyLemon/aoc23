mod days;

pub use crate::days::*;

pub fn get_input(day: &str, maybe_input: Option<String>) -> Result<String, Box<dyn std::error::Error>> {
    let (day, _part) = day.split_once('-').unwrap();
    let input_path = maybe_input.unwrap_or(format!("inputs/{}.txt", day));

    Ok(std::fs::read_to_string(input_path)?)
}
