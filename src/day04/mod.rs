pub mod input;
pub mod part1;
pub mod part2;

use crate::{Output, Part};

pub type Input = Vec<Camp>;

#[derive(Debug, Clone)]
pub struct Camp {
    first: Vec<u8>,
    second: Vec<u8>,
}

impl Camp {
    fn is_overlap(&self) -> bool {
        if self.first.len() <= self.second.len() {
            for section in self.first.iter() {
                if !self.second.contains(section) {
                    return false;
                }
            }
        } else {
            for section in self.second.iter() {
                if !self.first.contains(section) {
                    return false;
                }
            }
        }
        return true;
    }

    fn is_overlap_2(&self) -> bool {
        for section in self.first.iter() {
            if self.second.contains(section) {
                return true;
            }
        }
        for section in self.second.iter() {
            if self.first.contains(section) {
                return true;
            }
        }
        return false;
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
