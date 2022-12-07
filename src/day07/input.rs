use crate::day07::Input;

use super::AocFile;

const EXAMPLE_INPUT: &str = include_str!("../../input/07/example.txt");
const INPUT: &str = include_str!("../../input/07/input.txt");

pub fn read() -> Input {
    read_internal(INPUT)
}

pub fn read_example() -> Input {
    read_internal(EXAMPLE_INPUT)
}

pub fn read_internal(input: &str) -> Input {
    let commands: Vec<&str> = input.split("\n").collect();
    let mut current_path: Vec<&str> = vec![];
    let mut files: Vec<AocFile> = vec![];
    for command in commands {
        if command.contains("cd") {
            let path = &command[5..];
            if path == ".." {
                current_path.pop();
            } else if path != "/" {
                current_path.push(path)
            }
        }
        if !command.contains("$") && !command.contains("dir") {
            let file: Vec<&str> = command.split(" ").collect();
            let size: u32 = file[0].parse().unwrap();

            files.push(AocFile {
                size,
                path: current_path.join("/"),
            })
        }
    }
    return files;
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
