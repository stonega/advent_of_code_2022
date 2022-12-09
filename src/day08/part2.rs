use crate::day08::{Input, Output};

pub fn solve(input: &Input) -> Output {
    let (horizontal, vertical) = input;
    let mut result: u32 = 0;

    for i in 1..horizontal.len() - 1 {
        let trees = &horizontal[i];
        for j in 1..trees.len() - 1 {
            let v_trees = &vertical[j];
            let h_score = calc_score(&trees, j);
            let v_score = calc_score(&v_trees, i);
            if h_score * v_score > result {
                result = h_score * v_score
            }
        }
    }
    return result.into();
}

fn calc_score(tree: &Vec<u8>, index: usize) -> u32 {
    let i = tree[index];
    let mut left = 0;
    let mut right = 0;
    for d in tree[..index].iter().rev() {
        left += 1;
        if d >= &i {
            break;
        }
    }
    for d in tree[index + 1..].iter() {
        right += 1;
        if d >= &i {
            break;
        }
    }
    return left * right;
}
