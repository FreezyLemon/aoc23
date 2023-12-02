use aoc23::input::get_input;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let content = get_input()?;
    let games: Vec<Game> = content.lines().map(Game::parse).collect();

    let available_cubes = CubeSet {
        red: 12,
        blue: 14,
        green: 13,
    };

    let id_sum: u32 = games
        .iter()
        .filter(|g|
            g.sets.iter()
                .all(|set|
                    set.red <= available_cubes.red &&
                    set.blue <= available_cubes.blue &&
                    set.green <= available_cubes.green
                )
        )
        .map(|g| g.id)
        .sum();

    println!("{id_sum}");

    Ok(())
}

#[derive(Debug)]
struct Game {
    id: u32,
    sets: Vec<CubeSet>,
}

impl Game {
    pub fn parse(s: &str) -> Self {
        let (game_id, sets) = s.split_once(": ").expect("valid input");
        let id: u32 = game_id[5..].parse().expect("can parse as u32");
        let sets: Vec<CubeSet> = sets.split("; ").map(CubeSet::parse).collect();

        Self {
            id,
            sets,
        }
    }
}

#[derive(Debug)]
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
}

