use std::vec;

use crate::day10::{Input, Output};

use super::Instruction;

pub fn solve(input: &Input) -> Output {
    let mut x: i16 = 2;
    let mut cycle: i16 = 0;
    let mut result: String = "".to_string();
    for i in input {
        match i {
            Instruction::Noop => {
                cycle += 1;
                if cycle % 40 >= x - 1 && cycle % 40 <= x + 1 {
                    result.push('#');
                } else {
                    result.push('.')
                }
                if cycle % 40 == 0 {
                    result.push('\n')
                }
            }
            Instruction::Addx(time) => {
                for i in 0..2 {
                    cycle += 1;
                    if cycle % 40 >= x - 1 && cycle % 40 <= x + 1 {
                        result.push('#');
                    } else {
                        result.push('.')
                    }
                    if cycle % 40 == 0 {
                        result.push('\n')
                    }
                }
                x += time;
            }
        }
    }
    result.into()
}
