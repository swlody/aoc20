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

impl std::str::FromStr for Program {
    type Err = std::num::ParseIntError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let prog = input
            .lines()
            .map(|line| {
                let (instruction, value) = line.split_once(' ').unwrap();
                let value = value.parse()?;
                Ok(match instruction {
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

use std::convert::TryFrom;

impl Program {
    fn run(&mut self) -> bool {
        let mut visited = vec![false; self.prog.len()];
        let pc = usize::try_from(self.pc).expect("Invalid value for program counter");
        while let Some(instruction) = self.prog.get(pc) {
            if visited[pc] {
                return false;
            }
            visited[pc] = true;
            self.pc += 1;
            match instruction {
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
    for (idx, instr) in program.prog.iter().enumerate() {
        if let Acc(_) = instr {
            continue;
        }

        let mut program = program.clone();
        program.prog[idx] = match *instr {
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
