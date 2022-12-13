pub mod input;
pub mod part1;
pub mod part2;

use clap::builder::PossibleValue;

use crate::{Output, Part};

pub type Input = Vec<MoveStep>;

#[derive(Debug, Clone, PartialEq)]
pub enum Dir {
    Left,
    Right,
    Up,
    Down,
}

#[derive(PartialEq, Clone, Debug)]
pub struct Position {
    x: i16,
    y: i16,
}
trait Dedup<T: PartialEq + Clone> {
    fn clear_duplicates(&mut self);
}

impl<T: PartialEq + Clone> Dedup<T> for Vec<T> {
    fn clear_duplicates(&mut self) {
        let mut already_seen = Vec::new();
        self.retain(|item| match already_seen.contains(item) {
            true => false,
            _ => {
                already_seen.push(item.clone());
                true
            }
        })
    }
}


impl Position {
    fn move_position(step: &Dir, start: &Self) -> Self {
        let x = start.x.clone();
        let y = start.y.clone();
        match step {
            Dir::Left => Position { x: start.x - 1, y },
            Dir::Right => Position { x: x + 1, y },
            Dir::Up => Position { x, y: y + 1},
            Dir::Down => Position { x, y: y -1 },
        }
    }
}

#[derive(Debug, Clone)]
pub struct MoveStep {
    dir: Dir,
    num: u8,
}

impl MoveStep {
    fn try_from(from: &str) -> Self {
        let data: Vec<&str> = from.split(" ").collect();
        let dir = match data[0] {
            "R" => Dir::Right,
            "L" => Dir::Left,
            "U" => Dir::Up,
            "D" => Dir::Down,
            _ => Dir::Down,
        };
        let num = data[1].parse::<u8>().unwrap();
        MoveStep { dir, num }
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
