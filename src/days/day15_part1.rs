pub struct Day15Part1;

impl crate::days::Day for Day15Part1 {
    fn solve(&self, input: &str) -> String {
        input.split(',')
            .map(create_hash)
            .map(u32::from)
            .sum::<u32>()
            .to_string()
    }
}

fn create_hash(s: &str) -> u8 {
    let mut res: u16 = 0;

    for b in s.bytes() {
        res += u16::from(b);
        res *= 17;
        res %= 256;
    }

    u8::try_from(res).expect("value fits into u8")
}
