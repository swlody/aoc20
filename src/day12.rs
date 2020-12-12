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
    Move(Direction, i32),
    Forward(i32),
    Rotate(Turn),
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
            match (action, amount.parse().unwrap()) {
                ("N", amount) => Move(North, amount),
                ("S", amount) => Move(South, amount),
                ("E", amount) => Move(East, amount),
                ("W", amount) => Move(West, amount),

                ("F", amount) => Forward(amount),

                ("L" | "R", 180) => Rotate(Reverse),
                ("L", 90) | ("R", 270) => Rotate(Left),
                ("R", 90) | ("L", 270) => Rotate(Right),

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
            Move(North, val) => y += val,
            Move(South, val) => y -= val,
            Move(East, val) => x += val,
            Move(West, val) => x -= val,

            Forward(val) => match facing {
                North => y += val,
                South => y -= val,
                East => x += val,
                West => x -= val,
            },

            Rotate(turn) => facing.rotate(*turn),
        }
    }

    manhattan_distance((x, y), (0, 0))
}

pub fn solve_part2(actions: &[Action]) -> u32 {
    let (mut x, mut y) = (0, 0);
    let (mut waypoint_x, mut waypoint_y) = (10, 1);

    for action in actions.iter() {
        match action {
            Move(North, val) => waypoint_y += val,
            Move(South, val) => waypoint_y -= val,
            Move(East, val) => waypoint_x += val,
            Move(West, val) => waypoint_x -= val,

            Forward(val) => {
                y += val * waypoint_y;
                x += val * waypoint_x;
            }

            Rotate(Reverse) => {
                (waypoint_x, waypoint_y) = (-waypoint_x, -waypoint_y);
            }
            Rotate(Left) => {
                (waypoint_x, waypoint_y) = (-waypoint_y, waypoint_x);
            }
            Rotate(Right) => {
                (waypoint_x, waypoint_y) = (waypoint_y, -waypoint_x);
            }
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

    #[test]
    fn test_part2() {
        let input = input_generator(INPUT);
        assert_eq!(286, solve_part2(&input));
    }
}
