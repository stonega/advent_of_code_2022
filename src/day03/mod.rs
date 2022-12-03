pub mod input;
pub mod part1;
pub mod part2;

use std::{error::Error, io::Result, str::Bytes};

use nom::Slice;

use crate::{Output, Part};

pub type Input = Vec<RuckSacks>;

#[derive(Debug, Clone, PartialEq)]
pub struct RuckSacks {
    first: String,
    second: String,
    complete: String
}

impl RuckSacks {
    fn from_str(input: &str) -> RuckSacks {
        let length = input.len();
        let first = String::from(&input[0..length / 2]);
        let second = String::from(&input[length / 2..length]);
        let complete = String::from(input);
        return RuckSacks { first, second, complete };
    }

    fn find_same_item(&self) -> u16 {
        for item in self.first.chars() {
            if (self.second.contains(item)) {
                return item as u16;
            }
        }
        return 0;
    }
}

pub fn run(part: Part) -> Output {
    let input = match part {
        Part::One | Part::Two => input::read(),
        Part::ExampleOne | Part::ExampleTwo => input::read_example(),
    };

    match part {
        Part::ExampleOne | Part::One => part1::solve(&input),
        Part::ExampleTwo | Part::Two => part2::solve(&input),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_example_one() {
        let result = run(Part::ExampleOne);
        println!("{result}");
    }

    #[test]
    fn check_answer_one() {
        let result = run(Part::One);
        println!("{result}");
    }

    #[test]
    fn check_example_two() {
        let result = run(Part::ExampleTwo);
        println!("{result}");
    }

    #[test]
    fn check_answer_two() {
        let result = run(Part::Two);
        println!("{result}");
    }
}
