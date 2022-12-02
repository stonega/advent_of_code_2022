use crate::day02::*;

fn score(round: &Round) -> u32 {
    score_selection(round) + score_result(&round.result_needed)
}

pub fn solve(input: &Input) -> Output {
    input.iter().map(score).sum::<u32>().into()
}
