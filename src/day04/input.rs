use crate::day04::Input;

use super::Camp;

const EXAMPLE_INPUT: &str = include_str!("../../input/04/example.txt");
const INPUT: &str = include_str!("../../input/04/input.txt");

pub fn read() -> Input {
    read_internal(INPUT)
}

pub fn read_example() -> Input {
    read_internal(EXAMPLE_INPUT)
}

pub fn read_internal(input: &str) -> Input {
    input.trim().split("\n").map(parse_camp).collect()
}

fn parse_camp(input: &str) -> Camp {
    let data: Vec<&str> = input.split(",").collect();
    return Camp {
        first: str_to_vec(data[0]),
        second: str_to_vec(data[1]),
    };
}

fn str_to_vec(input: &str) -> Vec<u8> {
    let splitted: Vec<&str> = input.split("-").collect();
    let start: u8 = splitted[0].parse().unwrap();
    let end: u8 = splitted[1].parse().unwrap();
    (start..end + 1).collect()
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
