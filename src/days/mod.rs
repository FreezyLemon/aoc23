mod day01_part1;
mod day01_part2;
mod day01_part2_malox;
mod day02_part1;
mod day02_part2;
mod day03_part1;

pub use day01_part1::*;
pub use day01_part2::*;
pub use day01_part2_malox::*;
pub use day02_part1::*;
pub use day02_part2::*;
pub use day03_part1::*;

pub trait Day {
    fn solve(&self, input: String) -> String;
}

