use crate::day06::{Input, Output};

pub fn solve(input: &Input) -> Output {
    println!("{:?}", input.len());
    for i in 4..input.len() {
        if !check_same(&input[i - 4..i].to_vec()) {
            println!("{:?}", input[i - 4..i].to_vec());
            return (i as u32).into();
        }
    }
    panic!("Not solved")
}

fn check_same(input: &Vec<char>) -> bool {
    for i in 0..(input.len() - 1) {
        if input[i + 1..].to_vec().contains(&input[i]) {
            return true;
        }
    }
    return false;
}
