pub mod input;
pub mod part1;
pub mod part2;

use rug::Integer;

use crate::{Output, Part};

pub type Item = Integer;
pub type Input = (Vec<Vec<Item>>, Vec<Monkey>);

#[derive()]
pub struct Monkey {
    inspect: fn(item: &mut Item) -> (),
    test: fn(item: &Item) -> u8,
}

impl Monkey {
    fn inspect_operation(&self, items: &mut Vec<Item>) {
        for i in items {
            (self.inspect)(i)
        }
    }

    fn bored(&self, items: &mut Vec<Item>) {
        for i in items {
            *i /= 3;
        }
    }

    fn test_operation(&self, items: &Vec<Item>) -> Vec<(usize, Item)> {
        let mut result: Vec<(usize, Item)> = vec![];
        for i in items {
            let d = (self.test)(&i) as usize;
            result.push((d.try_into().unwrap(), i.clone()))
        }
        result
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
