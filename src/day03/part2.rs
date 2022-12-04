use crate::day03::{Input, Output};

use super::RuckSacks;

pub fn solve(input: &Input) -> Output {
    let mut result: u16 = 0;
    let groups: Vec<Vec<RuckSacks>> = input.chunks(3).map(|s| s.to_vec()).collect();
    for group in groups {
        for item in group[0].complete.chars() {
            if group[1].complete.contains(item) && group[2].complete.contains(item) {
                let item_num = item as u16;
                result += item_num;
                if item_num >= 97 {
                    result -= 96
                } else {
                    result -= 38
                }
                break;
            }
        }
    }
    return result.into();
}
