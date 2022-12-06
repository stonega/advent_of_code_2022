use crate::day05::{Input, Output};

pub fn solve(input: &Input) -> Output {
    let (stacks, actions) = input;
    let mut stack_clone = stacks.clone();
    for action in actions.iter() {
        for i in 0..action.amount {
            if let Some(index) = stack_clone[action.from as usize]
                .iter()
                .position(|x| *x != ' ')
            {
                let element = stack_clone[action.from as usize][index];

                stack_clone[action.to as usize].insert(i as usize, element);
                stack_clone[action.from as usize].remove(index);
            }
        }
    }
    let chars: Vec<char> = stack_clone
        .into_iter()
        .map(|s| found_first_char(s))
        .filter(|s| *s != '0')
        .collect();
    let s = String::from_iter(chars);
    s.into()
}

fn found_first_char(input: Vec<char>) -> char {
    if let Some(index) = input.iter().position(|x| *x != ' ') {
        return input[index];
    }
    return '0';
}