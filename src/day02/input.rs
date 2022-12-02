use crate::day02::{AttackType, Input, Round, RoundResult};

const EXAMPLE_INPUT: &str = include_str!("../../input/02/example.txt");
const INPUT: &str = include_str!("../../input/02/input.txt");

pub fn read() -> Input {
    read_internal(INPUT)
}

pub fn read_example() -> Input {
    read_internal(EXAMPLE_INPUT)
}

fn read_internal(input: &str) -> Input {
    input.trim().split('\n').map(try_parse_round).collect()
}

fn try_parse_round(value: &str) -> Round {
    let mut attacks = value.trim().split(' ');
    let theirs = match attacks.next() {
        Some("A") => Some(AttackType::Rock),
        Some("B") => Some(AttackType::Paper),
        Some("C") => Some(AttackType::Scissors),
        _ => None,
    };
    let unknown_attribute = attacks.next();
    let known_attribute = match unknown_attribute {
        Some("X") => Some((AttackType::Rock, RoundResult::Loss)),
        Some("Y") => Some((AttackType::Paper, RoundResult::Draw)),
        Some("Z") => Some((AttackType::Scissors, RoundResult::Win)),
        _ => None,
    };
    let (mine, result_needed) = known_attribute.unwrap();
    Round {
        theirs: theirs.unwrap(),
        mine,
        result_needed,
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_input() {
        let example_input = read_example();

        let first = example_input.first().unwrap();
        assert_eq!(first.theirs, AttackType::Rock);
        assert_eq!(first.mine, AttackType::Paper);
        assert_eq!(first.result_needed, RoundResult::Draw);

        let last = example_input.last().unwrap();
        assert_eq!(last.theirs, AttackType::Scissors);
        assert_eq!(last.mine, AttackType::Scissors);
        assert_eq!(last.result_needed, RoundResult::Win);
    }
}
