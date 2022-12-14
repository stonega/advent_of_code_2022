use crate::day11::{Input, Output};

use super::{Item, Monkey};

pub fn solve(input: &Input) -> Output {
    let (items, monkeys) = input;
    let mut items_input = items.clone();
    let mut result: Vec<u16> = vec![0; monkeys.len()];
    for i in 0..20 {
        let lens: Vec<u16>;
        (items_input, lens) = run_round(&items_input, monkeys);
        for i in 0..result.len() {
            result[i] += lens[i];
        }
        println!("{:?} {:?}", result, items_input);
    }
    println!("{:?}", result);
    get_business(&result)
}

fn run_round(items: &Vec<Vec<Item>>, monkeys: &Vec<Monkey>) -> (Vec<Vec<Item>>, Vec<u16>) {
    let mut items_input = items.clone();
    let mut result: Vec<u16> = vec![0; monkeys.len()];
    for i in 0..monkeys.len() {
        result[i] = items_input[i].len() as u16;
        monkeys[i].inspect_operation(&mut items_input[i]);
        monkeys[i].bored(&mut items_input[i]);
        let des = &monkeys[i].test_operation(&items_input[i]);
        for d in des {
            let (m, item) = d;
            items_input[*m].push(item.clone());
            items_input[i].clear();
        }
    }
    (items_input.clone(), result.clone())
}

fn get_business(result: &Vec<u16>) -> Output {
    let max = result.iter().max().unwrap();
    let result: Vec<u16> = result.iter().filter(|&s| s != max).cloned().collect();
    let max_next = result.iter().max().unwrap();
    (max * max_next).into()
}
