use crate::day10::{Input, Output};

use super::Instruction;

pub fn solve(input: &Input) -> Output {
    let mut x: i16 = 1;
    let mut cycle: i16 = 0;
    let mut result = 0;
    for i in input {
        match i {
            Instruction::Noop => {
                cycle += 1;
                if cycle % 40 == 20 {
                    result += cycle * x;
                    println!("{:?} {x}", cycle * x);
                }
            }
            Instruction::Addx(time) => {
                for i in 0..2 {
                    cycle += 1;
                    if cycle % 40 == 20 {
                        result += cycle * x;
                        println!("{:?} {x}", cycle * x);
                    }
                }
                x += time;
            }
        }
    }
    result.into()
}
