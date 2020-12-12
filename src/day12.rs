#[derive(Copy, Clone)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Copy, Clone)]
pub enum Turn {
    Left,
    Right,
    Reverse,
}

pub enum Action {
    Movement(Direction, i32),
    Forward(i32),
    Rotation(Turn),
}

use {Action::*, Direction::*, Turn::*};

impl Direction {
    fn rotate(&mut self, turn: Turn) {
        *self = match (*self, turn) {
            (North, Left) => West,
            (North, Right) => East,
            (North, Reverse) => South,

            (East, Left) => North,
            (East, Right) => South,
            (East, Reverse) => West,

            (South, Left) => East,
            (South, Right) => West,
            (South, Reverse) => North,

            (West, Left) => South,
            (West, Right) => North,
            (West, Reverse) => East,
        };
    }
}

pub fn input_generator(input: &str) -> Vec<Action> {
    input
        .lines()
        .map(|line| {
            let (action, amount) = line.split_at(1);
            let amount = amount.parse().unwrap();

            match (action, amount) {
                ("N", _) => Movement(North, amount),
                ("S", _) => Movement(South, amount),
                ("E", _) => Movement(East, amount),
                ("W", _) => Movement(West, amount),

                ("F", _) => Forward(amount),

                ("L" | "R", 180) => Rotation(Reverse),
                ("L", 90) | ("R", 270) => Rotation(Left),
                ("R", 90) | ("L", 270) => Rotation(Right),

                _ => panic!("Invalid input: {}", line),
            }
        })
        .collect()
}

fn manhattan_distance(a: (i32, i32), b: (i32, i32)) -> u32 {
    use std::convert::TryFrom;
    u32::try_from((b.0 - a.0).abs() + (b.1 - a.1).abs()).unwrap()
}

pub fn solve_part1(actions: &[Action]) -> u32 {
    let mut facing = East;
    let (mut x, mut y) = (0, 0);

    for action in actions.iter() {
        match action {
            Movement(North, val) => y += val,
            Movement(South, val) => y -= val,
            Movement(East, val) => x += val,
            Movement(West, val) => x -= val,

            Forward(val) => match facing {
                North => y += val,
                South => y -= val,
                East => x += val,
                West => x -= val,
            },

            Rotation(turn) => facing.rotate(*turn),
        }
    }

    manhattan_distance((x, y), (0, 0))
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = "F10
N3
F7
R90
F11";

    #[test]
    fn test_part1() {
        let input = input_generator(INPUT);
        assert_eq!(25, solve_part1(&input));
    }
}
