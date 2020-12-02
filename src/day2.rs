use aoc_runner_derive::aoc;

// oh my god this is so dumb

#[aoc(day2, part1)]
fn solve_part1(input: &str) -> usize {
    fn is_valid(input: &str) -> bool {
        // Ugly unwrapping because ugh
        let (range, rest) = input.split_once(' ').unwrap();
        let (start_range, end_range) = range.split_once('-').unwrap();
        let valid_range =
            start_range.parse::<usize>().ok().unwrap()..=end_range.parse::<usize>().ok().unwrap();
        let (character, password) = rest.split_once(':').unwrap();
        let password = password;
        valid_range.contains(&password.matches(character).count())
    }

    input.lines().filter(|line| is_valid(line)).count()
}

#[aoc(day2, part2)]
fn solve_part2(input: &str) -> usize {
    fn is_valid(input: &str) -> bool {
        let (indices, rest) = input.split_once(' ').unwrap();
        let (first_index, second_index) = indices.split_once('-').unwrap();
        let (character, password) = rest.split_once(':').unwrap();
        // `password` contains an extra space at the beginning so we don't need to account for 1-indexing
        // `character` is a `&str` so we use `starts_with()` to approximate equality because I'm lazy
        let result = character.starts_with(
            password
                .chars()
                .nth(first_index.parse::<usize>().unwrap())
                .unwrap(),
        ) ^ character.starts_with(
            password
                .chars()
                .nth(second_index.parse::<usize>().unwrap())
                .unwrap(),
        );

        result
    }

    input.lines().filter(|line| is_valid(line)).count()
}
