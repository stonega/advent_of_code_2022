use std::vec;

use rug::Integer;

use crate::day11::{Input, Item};

use super::Monkey;

const EXAMPLE_INPUT: &str = include_str!("../../input/11/example.txt");
const INPUT: &str = include_str!("../../input/11/input.txt");

pub fn read() -> Input {
    fn monkey_01_inspect(item: &mut Item) {
        *item *= 3
    }
    fn monkey_01_test(item: &Item) -> u8 {
        if item.mod_u(7) == 0 {
            7
        } else {
            1
        }
    }
    let monkey_01 = Monkey {
        inspect: monkey_01_inspect,
        test: monkey_01_test,
    };
    fn monkey_02_inspect(item: &mut Item) {
        *item *= 17;
    }
    fn monkey_02_test(item: &Item) -> u8 {
        if item.mod_u(19) == 0 {
            5
        } else {
            7
        }
    }
    let monkey_02 = Monkey {
        inspect: monkey_02_inspect,
        test: monkey_02_test,
    };
    fn monkey_03_inspect(item: &mut Item) {
        *item += 1;
    }
    fn monkey_03_test(item: &Item) -> u8 {
        if item.mod_u(13) == 0 {
            4
        } else {
            3
        }
    }
    let monkey_03 = Monkey {
        inspect: monkey_03_inspect,
        test: monkey_03_test,
    };
    fn monkey_04_inspect(item: &mut Item) {
        *item += 2;
    }
    fn monkey_04_test(item: &Item) -> u8 {
        if item.mod_u(3) == 0 {
            4
        } else {
            6
        }
    }
    let monkey_04 = Monkey {
        inspect: monkey_04_inspect,
        test: monkey_04_test,
    };
    fn monkey_05_inspect(item: &mut Item) {
        item.square_mut();
    }
    fn monkey_05_test(item: &Item) -> u8 {
        if item.mod_u(2) == 0 {
            0
        } else {
            6
        }
    }
    let monkey_05 = Monkey {
        inspect: monkey_05_inspect,
        test: monkey_05_test,
    };
    fn monkey_06_inspect(item: &mut Item) {
        *item += 8;
    }
    fn monkey_06_test(item: &Item) -> u8 {
        if item.mod_u(11) == 0 {
            2
        } else {
            3
        }
    }
    let monkey_06 = Monkey {
        inspect: monkey_06_inspect,
        test: monkey_06_test,
    };
    fn monkey_07_inspect(item: &mut Item) {
        *item += 6;
    }
    fn monkey_07_test(item: &Item) -> u8 {
        if item.mod_u(17) == 0 {
            0
        } else {
            1
        }
    }
    let monkey_07 = Monkey {
        inspect: monkey_07_inspect,
        test: monkey_07_test,
    };
    fn monkey_08_inspect(item: &mut Item) {
        *item += 7;
    }
    fn monkey_08_test(item: &Item) -> u8 {
        if item.mod_u(5) == 0 {
            2
        } else {
            5
        }
    }
    let monkey_08 = Monkey {
        inspect: monkey_08_inspect,
        test: monkey_08_test,
    };

    let monkeys = vec![
        monkey_01, monkey_02, monkey_03, monkey_04, monkey_05, monkey_06, monkey_07, monkey_08,
    ];
    let items: Vec<Vec<u32>> = vec![
        vec![93, 54, 69, 66, 71],
        vec![89, 51, 80, 66],
        vec![90, 92, 63, 91, 96, 63, 64],
        vec![65, 77],
        vec![76, 68, 94],
        vec![86, 65, 66, 97, 73, 83],
        vec![78],
        vec![89, 57, 59, 61, 87, 55, 55, 88],
    ];
    (to_u256_vec(&items), monkeys)
}

pub fn read_example() -> Input {
    fn monkey_01_inspect(item: &mut Item) {
        *item *= 19
    }
    fn monkey_01_test(item: &Item) -> u8 {
        if item.mod_u(23) == 0 {
            2
        } else {
            3
        }
    }
    let monkey_01 = Monkey {
        inspect: monkey_01_inspect,
        test: monkey_01_test,
    };
    fn monkey_02_inspect(item: &mut Item) {
        *item += 6;
    }
    fn monkey_02_test(item: &Item) -> u8 {
        if item.mod_u(19) == 0 {
            2
        } else {
            0
        }
    }
    let monkey_02 = Monkey {
        inspect: monkey_02_inspect,
        test: monkey_02_test,
    };
    fn monkey_03_inspect(item: &mut Item) {
        item.square_mut();
    }
    fn monkey_03_test(item: &Item) -> u8 {
        if item.mod_u(13) == 0 {
            1
        } else {
            3
        }
    }
    let monkey_03 = Monkey {
        inspect: monkey_03_inspect,
        test: monkey_03_test,
    };
    fn monkey_04_inspect(item: &mut Item) {
        *item += 3;
    }
    fn monkey_04_test(item: &Item) -> u8 {
        if item.mod_u(17) == 0 {
            0
        } else {
            1
        }
    }
    let monkey_04 = Monkey {
        inspect: monkey_04_inspect,
        test: monkey_04_test,
    };

    let monkeys = vec![monkey_01, monkey_02, monkey_03, monkey_04];
    let items = vec![
        vec![79, 98],
        vec![54, 65, 75, 74],
        vec![79, 60, 97],
        vec![74],
    ];
    (to_u256_vec(&items), monkeys)
}

fn to_u256_vec(input: &Vec<Vec<u32>>) -> Vec<Vec<Item>> {
    let mut result: Vec<Vec<Item>> = vec![];
    for i in input {
        result.push(to_u256(i));
    }
    result
}
fn to_u256(input: &Vec<u32>) -> Vec<Item> {
    let mut result: Vec<Item> = vec![];
    for i in input {
        result.push(Integer::from(*i));
    }
    result
}
#[cfg(test)]
mod test {
    use super::*;

    // #[test]
    // fn check_input() {
    //     let example_input = read_example();
    //     println!("{:?}", example_input);
    // }
}
