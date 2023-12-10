mod days;

pub use crate::days::*;

pub fn get_input(day: &str, maybe_input: Option<String>) -> Result<String, Box<dyn std::error::Error>> {
    let (day, _part) = day.trim_start_matches('d').split_once('p').expect("valid input");
    let input_path = maybe_input.unwrap_or(format!("inputs/day{:02}.txt", day.parse::<u32>().unwrap()));

    Ok(std::fs::read_to_string(input_path)?)
}
