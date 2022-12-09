use crate::day08::{Input, Output};

pub fn solve(input: &Input) -> Output {
    let (horizontal, vertical) = input;
    let mut result: u32 = 0;

    for i in 1..horizontal.len() - 1 {
        let trees = &horizontal[i];
        for j in 1..trees.len() - 1 {
            let v_trees = &vertical[j];
            let h_visible = check_visiable(&trees, j);
            let v_visible = check_visiable(&v_trees, i);
            if h_visible || v_visible {
                result += 1;
            }
        }
    }
    return (result + horizontal.len() as u32 * 2 + vertical.len() as u32 * 2 - 4).into();
}

fn check_visiable(tree: &Vec<u8>, index: usize) -> bool {
    let i = tree[index];
    let mut left = true;
    let mut right = true;
    for d in &tree[..index] {
        if d >= &i {
            left = false;
        }
    }
    for d in &tree[index + 1..] {
        if d >= &i {
            right = false;
        }
    }
    return left || right;
}
