use crate::day02::*;

pub fn score(round: &Round) -> u32 {
    score_selection(round) + score_result(&calculate_result(round))
}

pub fn solve(input: &Input) -> Output {
    input.iter().map(score).sum::<u32>().into()
}
