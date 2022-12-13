use crate::day09::{Input, Output, Dedup};

use super::{Dir, MoveStep, Position};

pub fn solve(input: &Input) -> Output {
    let mut start = Position { x: 0, y: 0 };
    let mut positions: Vec<Position> = vec![start.clone()];
    let mut direction: Dir = Dir::Left;
    for step in input {
        for j in 0..step.num {
            let direction = &step.dir;
            let next_position = Position::move_position(&direction, &start);
            if near_positions(&positions.last().unwrap(), &next_position) {
                positions.push(start.clone());
            }
            start = next_position;
        }
    }
    positions.clear_duplicates();
    let len = positions.len() as u32;
    len.into()
}

fn near_positions(first: &Position, second: &Position) -> bool{
    return first.x.abs_diff(second.x) > 1 || first.y.abs_diff(second.y) > 1
}

#[cfg(test)]
mod tests {
    use crate::day09::Position;
    use super::near_positions;
    fn check_near_positions() {
        let first = Position {x:10, y:-10};
        let second = Position {x:9, y:-10};
        let result = near_positions(&first, &second);
        println!("{result}");
    }
}