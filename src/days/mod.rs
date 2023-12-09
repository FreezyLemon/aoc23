mod day01_part1;
mod day01_part2;
mod day01_part2_malox;
mod day02_part1;
mod day02_part2;
mod day03_part1;
mod day03_part2;
mod day04_part1;
mod day04_part2;
mod day05_part1;
mod day05_part2;
mod day06_part1;
mod day06_part2;
mod day07_part1;
mod day07_part2;
mod day08_part1;
mod day08_part2;
mod day09_part1;
mod day09_part2;

pub use day01_part1::*;
pub use day01_part2::*;
pub use day01_part2_malox::*;
pub use day02_part1::*;
pub use day02_part2::*;
pub use day03_part1::*;
pub use day03_part2::*;
pub use day04_part1::*;
pub use day04_part2::*;
pub use day05_part1::*;
pub use day05_part2::*;
pub use day06_part1::*;
pub use day06_part2::*;
pub use day07_part1::*;
pub use day07_part2::*;
pub use day08_part1::*;
pub use day08_part2::*;
pub use day09_part1::*;
pub use day09_part2::*;

pub trait Day {
    fn solve(&self, input: String) -> String;
}

