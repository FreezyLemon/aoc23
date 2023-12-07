use std::collections::hash_map::HashMap;
use aoc23::*;

macro_rules! map_entry {
    ($k:literal, $v:expr) => {
        (String::from($k), Box::new($v) as Box<dyn Day>)
    };
}

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let days: HashMap<String, Box<dyn Day>> = HashMap::from([
        map_entry!("day01-part1", Day1Part1),
        map_entry!("day01-part2", Day1Part2),
        map_entry!("day01-part2-malox", Day1Part2Malox),
        map_entry!("day02-part1", Day2Part1),
        map_entry!("day02-part2", Day2Part2),
        map_entry!("day03-part1", Day3Part1),
        map_entry!("day04-part1", Day4Part1),
        map_entry!("day04-part2", Day4Part2),
        map_entry!("day05-part1", Day5Part1),
        map_entry!("day05-part2", Day5Part2),
        map_entry!("day06-part1", Day6Part1),
        map_entry!("day06-part2", Day6Part2),
        map_entry!("day07-part1", Day7Part1),
    ]);
        
    let mut args = std::env::args().skip(1);
    let Some(day) = args.next() else {
        return Err(Box::new(ProgramError::NotEnoughArguments));
    };

    let Some(day_impl) = days.get(&day) else {
        return Err(Box::new(ProgramError::NoImplementationFound));
    };

    let day_result = day_impl.solve(get_input(&day, args.next())?);
    println!("Result of {day}: {day_result}");

    Ok(())
}

#[derive(Debug)]
enum ProgramError {
    NotEnoughArguments,
    NoImplementationFound,
}

impl std::fmt::Display for ProgramError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        use ProgramError::*;

        write!(f, "{}", match self {
            NotEnoughArguments => "provide at least one argument",
            NoImplementationFound => "no implementation found (is it registered?)",
        })
    }
}

impl std::error::Error for ProgramError {}

