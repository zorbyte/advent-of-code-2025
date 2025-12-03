#[derive(Debug, PartialEq, Eq)]
pub enum Direction {
    Left,
    Right,
    None,
}

#[derive(Debug)]
pub struct Instruction(pub Direction, /* Magnitude: */ pub u64);

pub fn generate_instructions(input: String) -> Vec<Instruction> {
    input
        .lines()
        .map(|item| {
            let direction = match item.chars().next() {
                Some('R') => Direction::Right,
                Some('L') => Direction::Left,
                _ => panic!("expected instruction to be prefixed with either 'L' or 'R'"),
            };

            let magnitude = item[1..]
                .parse::<u64>()
                .expect("couldn't parse magnitude from instruction");

            Instruction(direction, magnitude)
        })
        .collect()
}
