use crate::day09::Input;

use super::MoveStep;

const EXAMPLE_INPUT: &str = include_str!("../../input/09/example.txt");
const INPUT: &str = include_str!("../../input/09/input.txt");

pub fn read() -> Input {
    read_internal(INPUT)
}

pub fn read_example() -> Input {
    read_internal(EXAMPLE_INPUT)
}

pub fn read_internal(input: &str) -> Input {
    input.split("\n").map(|s| MoveStep::try_from(s)).collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_input() {
        let example_input = read_example();
        println!("{:?}", example_input);
    }
}
