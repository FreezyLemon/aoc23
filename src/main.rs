use std::collections::hash_map::HashMap;

use aoc23::*;
use paste::paste;

macro_rules! make_day_string {
    ($d:literal, $p:literal) => {
        format!("d{}p{}", $d, $p)
    };
    ($d:literal, $p:literal, $suffix:expr) => {
        format!("{}-{}", make_day_string!($d, $p), std::primitive::str::to_lowercase($suffix))
    };
}

macro_rules! make_day_struct {
    ($d:literal, $p:literal $(, $suffix:literal)?) => {
        paste! { [<Day $d Part $p $($suffix)?>] }
    };
}

macro_rules! map_entry {
    ($d:literal, $p:literal $(, $suffix:literal)?) => {
        (make_day_string!($d, $p $(, $suffix)?), Box::new(make_day_struct!($d, $p $(, $suffix)?)) as Box<dyn Day>)
    };
}

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let days: HashMap<String, Box<dyn Day>> = HashMap::from([
        map_entry!(1, 1),
        map_entry!(1, 2),
        map_entry!(1, 2, "Malox"),
        map_entry!(2, 1),
        map_entry!(2, 2),
        map_entry!(3, 1),
        map_entry!(3, 2),
        map_entry!(4, 1),
        map_entry!(4, 2),
        map_entry!(5, 1),
        map_entry!(5, 2),
        map_entry!(6, 1),
        map_entry!(6, 2),
        map_entry!(7, 1),
        map_entry!(7, 2),
        map_entry!(8, 1),
        map_entry!(8, 2),
        map_entry!(9, 1),
        map_entry!(9, 2),
        map_entry!(10, 1),
        map_entry!(10, 2),
        map_entry!(11, 1),
    ]);
        
    let mut args = std::env::args().skip(1);
    let Some(day) = args.next() else {
        return Err(Box::new(ProgramError::NotEnoughArguments));
    };

    if day == "-h" || day == "--help" {
        println!("available implementations:");
        println!("{:?}", days.into_keys().collect::<Vec<_>>());
        return Ok(());
    }

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

