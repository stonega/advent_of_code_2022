pub mod input;
pub mod part1;
pub mod part2;

use crate::{Output, Part};

#[derive(Debug, PartialEq)]
pub enum AttackType {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug, PartialEq)]
pub enum RoundResult {
    Win,
    Loss,
    Draw,
}

pub struct Round {
    theirs: AttackType,
    mine: AttackType,
    result_needed: RoundResult,
}

pub type Input = Vec<Round>;

fn score_selection(round: &Round) -> u32 {
    match round.mine {
        AttackType::Rock => 1,
        AttackType::Paper => 2,
        AttackType::Scissors => 3,
    }
}

fn score_result(result: &RoundResult) -> u32 {
    match result {
        RoundResult::Win => 6,
        RoundResult::Loss => 0,
        RoundResult::Draw => 3,
    }
}

fn calculate_result(round: &Round) -> RoundResult {
    if round.theirs == round.mine {
        return RoundResult::Draw;
    }
    if they_won(round) {
        return RoundResult::Loss;
    }
    return RoundResult::Win;
}

fn they_won(round: &Round) -> bool {
    match round.theirs {
        AttackType::Rock => round.mine == AttackType::Scissors,
        AttackType::Paper => round.mine == AttackType::Rock,
        AttackType::Scissors => round.mine == AttackType::Paper,
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
    }

    #[test]
    fn check_answer_one() {
        let result = run(Part::One);
        assert_eq!(result, 13_221)
    }

    #[test]
    fn check_answer_two() {
        let result = run(Part::Two);
        println!("{result}");
    }
}
