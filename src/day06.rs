use std::collections::HashSet;

pub fn solve_part1(input: &str) -> usize {
    let groups = input.split("\n\n");

    let mut total = 0;
    for group in groups {
        let people = group.lines();
        let mut set = HashSet::<char>::new();

        for person in people {
            for answer in person.chars() {
                set.insert(answer);
            }
        }

        total += set.len();
    }

    total
}

pub fn solve_part2(input: &str) -> usize {
    let groups = input.split("\n\n");

    let mut total = 0;
    for group in groups {
        let mut people = group.lines();
        let mut set = HashSet::<char>::new();
        let first_person = people.next().unwrap();
        for answer in first_person.chars() {
            set.insert(answer);
        }

        while let Some(person) = people.next() {
            let mut new_set = HashSet::<char>::new();
            for answer in person.chars() {
                new_set.insert(answer);
            }
            set = set.intersection(&new_set).copied().collect();
        }

        total += set.len();
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = "abc

a
b
c

ab
ac

a
a
a
a

b";

    #[test]
    fn test_part1() {
        assert_eq!(11, solve_part1(&INPUT));
    }

    #[test]
    fn test_part2() {
        assert_eq!(6, solve_part2(&INPUT));
    }
}
