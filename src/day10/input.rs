use crate::day10::Input;

use super::Instruction;

const EXAMPLE_INPUT: &str = include_str!("../../input/10/example.txt");
const INPUT: &str = include_str!("../../input/10/input.txt");

pub fn read() -> Input {
    read_internal(INPUT)
}

pub fn read_example() -> Input {
    read_internal(EXAMPLE_INPUT)
}

pub fn read_internal(input: &str) -> Input {
    let data: Vec<&str> = input.split("\n").collect();
    let mut result: Vec<Instruction> = vec![];
    for i in data {
        if i == "noop" {
            result.push(Instruction::Noop);
        } else {
            let x: Vec<&str> = i.split(" ").collect();
            let y = x[1].parse::<i16>().unwrap();
            result.push(Instruction::Addx((y)));
        }
    }
    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_input() {
        let example_input = read_example();
        println!("{:?}", example_input);
    }
}
