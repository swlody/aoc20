use std::str::FromStr;

#[derive(Debug, Clone)]
enum Instruction {
    Nop(i32),
    Acc(i32),
    Jmp(i32),
}

use Instruction::*;

#[derive(Clone)]
pub struct Program {
    prog: Vec<Instruction>,
    pc: usize,
    acc: i32,
}

impl FromStr for Program {
    type Err = std::num::ParseIntError;

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

pub fn input_generator(input: &str) -> Program {
    input.parse().unwrap()
}

pub fn solve_part1(program: &Program) -> i32 {
    let mut program = program.clone();
    let mut visited: Vec<bool> = vec![false; program.prog.len()];
    while let Some(instr) = program.prog.get(program.pc) {
        if visited[program.pc] {
            break;
        }
        visited[program.pc] = true;
        program.pc += 1;
        match instr {
            Nop(_val) => {}
            Acc(val) => program.acc += val,
            Jmp(val) => program.pc = program.pc.wrapping_add(*val as usize) - 1,
        }
    }
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

        let mut visited: Vec<bool> = vec![false; program.prog.len()];
        let mut terminated = true;
        while let Some(instr) = program.prog.get(program.pc) {
            if visited[program.pc] {
                terminated = false;
                break;
            }
            visited[program.pc] = true;
            program.pc += 1;
            match instr {
                Nop(_val) => {}
                Acc(val) => program.acc += val,
                Jmp(val) => program.pc = program.pc.wrapping_add(*val as usize) - 1,
            }
        }

        if terminated {
            return program.acc;
        }
    }

    0
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
