use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::BTreeSet;

#[aoc_generator(day1)]
fn input_generator(input: &str) -> BTreeSet<u32> {
    input.lines().map(|x| x.parse::<u32>().unwrap()).collect()
}

#[aoc(day1, part1)]
fn solve_part1(set: &BTreeSet<u32>) -> u32 {
    for num in set.iter() {
        if let Some(result) = set.get(&(2020 - num)) {
            return num * result;
        }
    }
    panic!("No solution");
}
