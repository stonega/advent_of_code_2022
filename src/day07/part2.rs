use std::collections::HashMap;

use crate::day07::{Input, Output};

pub fn solve(input: &Input) -> Output {
    let mut dirs: HashMap<String, u32> = HashMap::new();
    let mut total: u32 = 0;
    for file in input {
        total += file.size;
        let paths = get_paths(&file.path);
        for p in paths {
            let size = dirs.get(&p);
            match size {
                Some(i) => dirs.insert(p, i + file.size),
                None => dirs.insert(p, file.size),
            };
        }
    }
    println!("{}", total);
    let unused = 70000000 - total;
    let clean = 30000000 - unused;
    let mut result: u32 = total;
    for (_, size) in dirs {
        if size >= clean && size < result {
            result = size
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
        if input != "" {
            output_string.push(String::from(""));
        }
    }
    return output_string;
}
