struct Policy {
    position: (usize, usize),
    character: char,
    password: String,
}

#[aoc_generator(day2)]
fn input_generator(input: &str) -> Option<Vec<Policy>> {
    input
        .lines()
        .map(|line| {
            let (range, rest) = line.split_once(' ')?;
            let (first, second) = range.split_once('-')?;
            let (character, password) = rest.split_once(": ")?;
            if character.len() != 1 {
                return None;
            }

            Some(Policy {
                position: (first.parse().ok()?, second.parse().ok()?),
                character: character.as_bytes()[0] as char,
                password: password.to_owned(),
            })
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
            let first_character = policy.password.as_bytes()[first_index - 1] as char;
            let second_character = policy.password.as_bytes()[second_index - 1] as char;
            (first_character == policy.character) ^ (second_character == policy.character)
        })
        .count()
}
