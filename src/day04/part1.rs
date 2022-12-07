use crate::day04::{Input, Output};

pub fn solve(input: &Input) -> Output {
    let mut result: u16 = 0;
    for camp in input.iter() {
        if camp.is_overlap() {
            result += 1;
        }
    }
    result.into()
}
