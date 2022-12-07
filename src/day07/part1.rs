use std::collections::HashMap;

use crate::day07::{Input, Output};

pub fn solve(input: &Input) -> Output {
    println!("{:?}", input);
    let mut dirs: HashMap<String, u32> = HashMap::new();
    for file in input {
        let paths = get_paths(&file.path);
        for p in paths {
            let size = dirs.get(&p);
            match size {
                Some(i) => dirs.insert(p, i + file.size),
                None => dirs.insert(p, file.size),
            };
        }
    }
    let mut result: u32 = 0;
    println!("{:?}", dirs);
    for (_, size) in dirs {
        if size <= 100000 {
            result += size
        }
    }
    result.into()
}

fn get_paths(input: &String) -> Vec<String> {
    let r: Vec<&str> = input.split("/").collect();

    let mut output_string: Vec<String> = Vec::new();

    for i in 0..r.len() {
        if i == 0 {
            output_string.push(String::from(r[i]));
        } else {
            output_string.push(r[..i + 1].join("/"));
        }
    }
    if input != "" {
        output_string.push(String::from(""));
    }
    return output_string;
}
