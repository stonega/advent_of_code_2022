use crate::day08::Input;

const EXAMPLE_INPUT: &str = include_str!("../../input/08/example.txt");
const INPUT: &str = include_str!("../../input/08/input.txt");

pub fn read() -> Input {
    read_internal(INPUT)
}

pub fn read_example() -> Input {
    read_internal(EXAMPLE_INPUT)
}

pub fn read_internal(input: &str) -> Input {
    let mut horizontal: Vec<Vec<u8>> = Vec::new();
    let mut vertical: Vec<Vec<u8>> = Vec::new();
    let data: Vec<&str> = input.split("\n").collect();
    for i in &data {
        let trees: Vec<u8> = i.chars().map(|s| s as u8 - 48).collect();
        horizontal.push(trees);
    }
    for i in 0..data[0].len() {
        let tress: Vec<u8> = data
            .iter()
            .map(|s| s.chars().nth(i).unwrap() as u8 - 48)
            .collect();
        vertical.push(tress);
    }
    return (horizontal, vertical);
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
