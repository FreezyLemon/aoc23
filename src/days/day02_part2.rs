pub struct Day2Part2;

impl crate::days::Day for Day2Part2 {
    fn solve(&self, input: String) -> String { 
        input.lines()
            .map(|l| l.split_once(": ").expect("line has :"))
            .map(|(_, cubes)| minimum_cubeset(cubes))
            .map(CubeSet::power)
            .sum::<u32>()
            .to_string()
    }
}

struct CubeSet {
    red: u8,
    green: u8,
    blue: u8,
}

impl CubeSet {
    fn empty() -> Self {
        Self {
            red: 0,
            green: 0,
            blue: 0,
        }
    }

    fn power(self) -> u32 {
        self.red as u32 * self.green as u32 * self.blue as u32
    }
}

fn minimum_cubeset(s: &str) -> CubeSet {
    s.split([',', ';'])
        .map(|color| color.trim_start().split_once(' ').unwrap())
        .map(|(amount, color)| (amount.parse::<u8>().unwrap(), color))
        .fold(CubeSet::empty(), |mut set, (amount, color)| {
            match color {
                "red" if amount > set.red => set.red = amount,
                "green" if amount > set.green => set.green = amount,
                "blue" if amount > set.blue => set.blue = amount,
                _ => {}
            }

            set
        })
}

