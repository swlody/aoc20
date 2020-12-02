use aoc_runner_derive::{aoc, aoc_generator};

struct Policy {
    position: (usize, usize),
    character: char,
    password: String,
}

#[aoc_generator(day2)]
fn input_generator(input: &str) -> Vec<Policy> {
    input
        .lines()
        .map(|line| {
            let (range, rest) = line.split_once(' ').unwrap();
            let (first, second) = range.split_once('-').unwrap();
            let (character, password) = rest.split_once(": ").unwrap();
            assert!(character.len() == 1);

            Policy {
                position: (first.parse().unwrap(), second.parse().unwrap()),
                character: character.chars().next().unwrap(),
                password: password.to_owned(),
            }
        })
        .collect()
}

#[aoc(day2, part1)]
fn solve_part1(policies: &[Policy]) -> usize {
    policies
        .iter()
        .filter(|policy| {
            let (start_range, end_range) = policy.position;
            let num_matches = policy.password.matches(policy.character).count();
            (start_range..=end_range).contains(&num_matches)
        })
        .count()
}

#[aoc(day2, part2)]
fn solve_part2(policies: &[Policy]) -> usize {
    policies
        .iter()
        .filter(|policy| {
            let (first_index, second_index) = policy.position;
            let first_character = policy.password.chars().nth(first_index - 1).unwrap();
            let second_character = policy.password.chars().nth(second_index - 1).unwrap();
            (first_character == policy.character) ^ (second_character == policy.character)
        })
        .count()
}
