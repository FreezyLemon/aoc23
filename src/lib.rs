mod days;

pub use crate::days::*;

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
    let raw_input = match maybe_input {
        Some(input) => input?,
        None => ALL_INPUTS.iter()
            .find(|(file, _)| file == &format!("day{:02}.txt", day.parse::<u32>().unwrap()))
            .map(|(_, content)| content.to_string())
            .expect("default input available in binary")
    };

    // This allows us to always assume '\n' line endings
    Ok(raw_input.replace("\r\n", "\n"))
}
