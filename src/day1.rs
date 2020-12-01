use aoc_runner_derive::{aoc, aoc_generator};
use std::{collections::BTreeSet, num::ParseIntError};

#[aoc_generator(day1)]
fn input_generator(input: &str) -> Result<BTreeSet<u32>, ParseIntError> {
    input.lines().map(|line| line.parse()).collect()
}

fn get_pair(set: &BTreeSet<u32>, sum: u32) -> Option<(u32, u32)> {
    for &num in set.iter() {
        if let Some(&result) = set.get(&(sum - num)) {
            return Some((num, result));
        }
    }
    None
}

#[aoc(day1, part1)]
fn solve_part1(set: &BTreeSet<u32>) -> Option<u32> {
    let (a, b) = get_pair(&set, 2020)?;
    Some(a * b)
}

#[aoc(day1, part2)]
fn solve_part2(set: &BTreeSet<u32>) -> Option<u32> {
    for &num in set.iter() {
        if let Some((a, b)) = get_pair(&set, 2020 - num) {
            return Some(a * b * num);
        }
    }
    None
}
