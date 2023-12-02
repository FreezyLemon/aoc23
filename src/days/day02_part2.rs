use std::cmp::max;

pub struct Day2Part2;

impl crate::days::Day for Day2Part2 {
    fn solve(&self, input: String) -> String {
        let games: Vec<Game> = input.lines().map(Game::parse).collect();

        let set_power_sum: u32 = games
            .iter()
            .map(|g|
                 g.sets.iter()
                    .fold(CubeSet::default(),
                        |acc, s| {
                            CubeSet {
                                red: max(acc.red, s.red),
                                blue: max(acc.blue, s.blue),
                                green: max(acc.green, s.green),
                            }
                        }
                    )
                    .power()
            )
            .sum();

        set_power_sum.to_string()
    }
}

#[derive(Debug)]
struct Game {
    _id: u32,
    sets: Vec<CubeSet>,
}

impl Game {
    pub fn parse(s: &str) -> Self {
        let (game_id, sets) = s.split_once(": ").expect("valid input");
        let id: u32 = game_id[5..].parse().expect("can parse as u32");
        let sets: Vec<CubeSet> = sets.split("; ").map(CubeSet::parse).collect();

        Self {
            _id: id,
            sets,
        }
    }
}

#[derive(Debug, Default)]
struct CubeSet {
    red: u32,
    blue: u32,
    green: u32,
}

impl CubeSet {
    pub fn parse(s: &str) -> Self {
        let mut cubes = Self {
            red: 0,
            blue: 0,
            green: 0,
        };

        for color in s.split(", ") {
            let (amount, color) = color.split_once(' ').unwrap();
            let amount: u32 = amount.parse().unwrap();
            match color {
                "red" => cubes.red = amount,
                "blue" => cubes.blue = amount,
                "green" => cubes.green = amount,
                c => panic!("unexpected color name {c}"),
            }
        }

        cubes
    }

    pub fn power(&self) -> u32 {
        self.red * self.blue * self.green
    }
}

