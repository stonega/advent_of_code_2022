use super::{Dedup, Dir, MoveStep, Position};
use crate::day09::{Input, Output};

pub fn solve(input: &Input) -> Output {
    let mut start = Position { x: 0, y: 0 };
    let mut positions: Vec<Position> = vec![start.clone()];
    let mut direction: Dir = Dir::Left;
    for step in input {
        for j in 0..step.num {
            let direction = &step.dir;
            start = Position::move_position(&direction, &start);
            positions.push(start.clone());
        }
    }

    for i in 0..9 {
        println!("{:?}", positions.len());
        positions = get_positions(&positions);
    }

    positions.clear_duplicates();
    let len = positions.len() as u32;
    len.into()
}

fn get_positions(ps: &Vec<Position>) -> Vec<Position> {
    let start = &ps[0];
    let mut positions: Vec<Position> = vec![start.clone()];
    for i in 1..ps.len() - 1 {
        if near_positions(&positions.last().unwrap(), &ps[i + 1]) {
            positions.push(ps[i].clone());
        }
    }
    positions
}

fn near_positions(first: &Position, second: &Position) -> bool {
    return first.x.abs_diff(second.x) > 1 || first.y.abs_diff(second.y) > 1;
}

fn check_position(source: &Position, current: &Position, next: &Position) -> Position {
    if source.x == current.x || source.y == current.y {
        return current.clone();
    }
    current.clone()
}
