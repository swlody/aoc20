pub struct Policy<'a> {
    position: (usize, usize),
    character: char,
    password: &'a str,
}

pub fn input_generator(input: &str) -> Vec<Policy> {
    input
        .lines()
        .map(|line| {
            let (range, rest) = line.split_once(' ').unwrap();
            let (first, second) = range.split_once('-').unwrap();
            let (character, password) = rest.split_once(": ").unwrap();
            assert!(character.is_ascii() && character.len() == 1);

            Policy {
                position: (first.parse().unwrap(), second.parse().unwrap()),
                character: character.as_bytes()[0] as char,
                password,
            }
        })
        .collect()
}

pub fn solve_part1(policies: &[Policy]) -> usize {
    policies
        .iter()
        .filter(|policy| {
            let (start_range, end_range) = policy.position;
            let num_matches = policy.password.matches(policy.character).count();
            (start_range..=end_range).contains(&num_matches)
        })
        .count()
}

pub fn solve_part2(policies: &[Policy]) -> usize {
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

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = "1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc";

    #[test]
    fn test_part1() {
        let policies = input_generator(INPUT);
        assert_eq!(2, solve_part1(&policies));
    }

    #[test]
    fn test_part2() {
        let policies = input_generator(INPUT);
        assert_eq!(1, solve_part2(&policies));
    }
}
