mod days;

pub use crate::days::*;

#[cfg(unix)]
pub const LINE_SEPARATOR: &str = "\n";
#[cfg(not(unix))]
pub const LINE_SEPARATOR: &str = "\r\n";

#[cfg(unix)]
pub const DOUBLE_LINE_SEPARATOR: &str = "\n\n";
#[cfg(not(unix))]
pub const DOUBLE_LINE_SEPARATOR: &str = "\r\n\r\n";

pub fn get_input(
    day: &str,
    maybe_input: Option<String>,
) -> Result<String, Box<dyn std::error::Error>> {
    let (day, _part) = day
        .trim_start_matches('d')
        .split_once('p')
        .expect("valid input");
    let input_path =
        maybe_input.unwrap_or(format!("inputs/day{:02}.txt", day.parse::<u32>().unwrap()));

    Ok(std::fs::read_to_string(input_path)?)
}
