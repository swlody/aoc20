struct Policy<'a> {
    position: (usize, usize),
    character: char,
    password: &'a str,
}

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
                password,
            })
        })
        .collect()
}

#[aoc(day2, part1)]
fn solve_part1(input: &str) -> Option<usize> {
    let policies = input_generator(&input)?;
    let num_valid_policies = policies
        .iter()
        .filter(|policy| {
            let (start_range, end_range) = policy.position;
            let num_matches = policy.password.matches(policy.character).count();
            (start_range..=end_range).contains(&num_matches)
        })
        .count();
    Some(num_valid_policies)
}

#[aoc(day2, part2)]
fn solve_part2(input: &str) -> Option<usize> {
    let policies = input_generator(&input)?;
    let num_valid_policies = policies
        .iter()
        .filter(|policy| {
            let (first_index, second_index) = policy.position;
            let first_character = policy.password.as_bytes()[first_index - 1] as char;
            let second_character = policy.password.as_bytes()[second_index - 1] as char;
            (first_character == policy.character) ^ (second_character == policy.character)
        })
        .count();
    Some(num_valid_policies)
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = "1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc";

    #[test]
    fn test_part1() {
        assert_eq!(2, solve_part1(&INPUT).unwrap());
    }

    #[test]
    fn test_part2() {
        assert_eq!(1, solve_part2(&INPUT).unwrap());
    }
}
