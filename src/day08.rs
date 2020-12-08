#[derive(Clone)]
enum Instruction {
    Nop(i32),
    Acc(i32),
    Jmp(i32),
}

#[derive(Clone)]
pub struct Program {
    prog: Vec<Instruction>,
    pc: i32,
    acc: i32,
}

use Instruction::*;

use std::{num::ParseIntError, str::FromStr};

impl FromStr for Program {
    type Err = ParseIntError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let prog = input
            .lines()
            .map(|line| {
                let (instr, value) = line.split_once(' ').unwrap();
                let value = value.parse()?;
                Ok(match instr {
                    "nop" => Nop(value),
                    "acc" => Acc(value),
                    "jmp" => Jmp(value),
                    unknown => panic!("Invalid instruction in input: {}", unknown),
                })
            })
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self {
            prog,
            pc: 0,
            acc: 0,
        })
    }
}

impl Program {
    fn run(&mut self) -> bool {
        let mut visited = vec![false; self.prog.len()];
        while let Some(instr) = self.prog.get(self.pc as usize) {
            if visited[self.pc as usize] {
                return false;
            }
            visited[self.pc as usize] = true;
            self.pc += 1;
            match instr {
                Nop(_val) => {}
                Acc(val) => self.acc += val,
                Jmp(val) => self.pc += val - 1,
            }
        }
        true
    }
}

pub fn input_generator(input: &str) -> Program {
    input.parse().unwrap()
}

pub fn solve_part1(program: &Program) -> i32 {
    let mut program = program.clone();
    program.run();
    program.acc
}

pub fn solve_part2(program: &Program) -> i32 {
    for i in 0..program.prog.len() {
        if let Acc(_) = program.prog[i] {
            continue;
        }

        let mut program = program.clone();
        program.prog[i] = match program.prog[i] {
            Jmp(val) => Nop(val),
            Nop(val) => Jmp(val),
            Acc(_) => unreachable!(),
        };

        let terminated = program.run();
        if terminated {
            return program.acc;
        }
    }
    panic!("No valid replacement found");
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";

    #[test]
    fn test_part1() {
        let program = input_generator(INPUT);
        assert_eq!(5, solve_part1(&program));
    }

    #[test]
    fn test_part2() {
        let program = input_generator(INPUT);
        assert_eq!(8, solve_part2(&program));
    }
}
