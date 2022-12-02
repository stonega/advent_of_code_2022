use crate::day02::*;

fn calculate_attack(round: &Round) -> AttackType {
    match round.result_needed {
        RoundResult::Win => match round.theirs {
            AttackType::Rock => AttackType::Paper,
            AttackType::Paper => AttackType::Scissors,
            AttackType::Scissors => AttackType::Rock,
        },
        RoundResult::Loss => match round.theirs {
            AttackType::Rock => AttackType::Scissors,
            AttackType::Paper => AttackType::Rock,
            AttackType::Scissors => AttackType::Paper,
        },
        RoundResult::Draw => match round.theirs {
            AttackType::Rock => AttackType::Rock,
            AttackType::Paper => AttackType::Paper,
            AttackType::Scissors => AttackType::Scissors,
        },
    }
}

fn score(round: &Round) -> u32 {
    score_attack(&calculate_attack(round)) + score_result(&round.result_needed)
}

pub fn solve(input: &Input) -> Output {
    input.iter().map(score).sum::<u32>().into()
}
