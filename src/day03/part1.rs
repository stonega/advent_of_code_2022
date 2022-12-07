use crate::day03::{Input, Output};

pub fn solve(input: &Input) -> Output {
    let mut result: u16 = 0;
    for rucksacks in input.iter() {
        let item = rucksacks.find_same_item();
        result += item;
        if item >= 97 {
            result -= 96
        } else {
            result -= 38
        }
    }
    return result.into();
}
