use crate::day02::*;

fn calculate_result(round: &Round) -> RoundResult {
    if round.theirs == round.mine {
        return RoundResult::Draw;
    }
    if they_won(round) {
        return RoundResult::Loss;
    }
    return RoundResult::Win;
}

pub fn score(round: &Round) -> u32 {
    score_attack(&round.mine) + score_result(&calculate_result(round))
}

pub fn solve(input: &Input) -> Output {
    input.iter().map(score).sum::<u32>().into()
}
