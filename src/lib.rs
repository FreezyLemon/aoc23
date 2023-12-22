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

const ALL_INPUTS: &[(&str, &str)] =  &include!(concat!(env!("OUT_DIR"), "/inputs.rs"));

pub fn get_input(
    day: &str,
    maybe_input: Option<String>,
) -> Result<String, Box<dyn std::error::Error>> {
    let (day, _part) = day
        .trim_start_matches('d')
        .split_once('p')
        .expect("valid input");
    
    let maybe_input = maybe_input.map(std::fs::read_to_string);

    Ok(match maybe_input {
        Some(input) => input?,
        None => ALL_INPUTS.iter()
            .find(|(file, _)| file == &format!("day{:02}.txt", day.parse::<u32>().unwrap()))
            .map(|(_, content)| content.to_string())
            .expect("default input available in binary")
    })
}
