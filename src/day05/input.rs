use crate::day05::Input;

use super::{Action, CrateStack};

const EXAMPLE_INPUT: &str = include_str!("../../input/05/example.txt");
const INPUT: &str = include_str!("../../input/05/input.txt");

pub fn read() -> Input {
    read_internal(INPUT)
}

pub fn read_example() -> Input {
    read_internal(EXAMPLE_INPUT)
}

pub fn read_internal(input: &str) -> Input {
    let values: Vec<&str> = input.split("\n\n").collect();
    let actions: Vec<Action> = values[1].split("\n").map(|a| parse_action(a)).collect();
    let mut horizontal_data: Vec<Vec<char>> =
        values[0].split("\n").map(|a| parse_stack(a)).collect();
    horizontal_data.truncate(horizontal_data.len() - 1);
    let stacks = convert_stacks(&horizontal_data);
    return (stacks, actions);
}

fn parse_action(input: &str) -> Action {
    let mut from = 0;
    let mut to = 0;
    let mut amount = input.as_bytes()[5] - 48;
    let second = input.as_bytes()[6];
    if (second > 32) {
        amount = amount * 10 + second - 48;
        from = input.as_bytes()[13] - 49;
        to = input.as_bytes()[18] - 49;
    } else {
        from = input.as_bytes()[12] - 49;
        to = input.as_bytes()[17] - 49;
    }
    Action { from, to, amount }
}

fn parse_stack(input: &str) -> Vec<char> {
    let mut result: Vec<char> = vec![];
    for i in 0..input.len() - 1 {
        if i % 4 == 1 {
            result.push(input.chars().nth(i).unwrap())
        }
    }
    result
}

fn convert_stacks(input: &Vec<Vec<char>>) -> Vec<CrateStack> {
    let mut result: Vec<CrateStack> = vec![];
    for n in 0..input[0].len() {
        result.push(vec![]);
        for d in input {
            if d.len() > 0 {
                result[n].push(d[n])
            }
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
