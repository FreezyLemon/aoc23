use std::{env, error::Error, fmt, path::PathBuf, str::FromStr};

pub fn get_input() -> Result<String, Box<dyn std::error::Error>> {
    // skip first arg (binary name)
    let mut args = env::args();
    let bin_name = args.next().unwrap();
    let bin_file = PathBuf::from_str(&bin_name)?;
    let bin_file_name = bin_file.file_name().unwrap().to_string_lossy();

    let (day, _part) = bin_file_name.split_once('-').expect("binary name contains '-'");

    let input_path = if let Some(arg) = args.next() {
        arg
    } else {
        format!("inputs/{}.txt", day) // convenience
    };

    Ok(std::fs::read_to_string(input_path)?)
}

#[derive(Debug)]
struct ArgsError;

impl fmt::Display for ArgsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "not enough arguments provided")
    }
}

impl Error for ArgsError {}

