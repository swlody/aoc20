pub fn solve_part1(input: &str) -> u32 {
    input
        .split("\n\n")
        .map(|group| {
            group
                .lines()
                .flat_map(str::as_bytes)
                .fold(0u32, |set, c| set | 1 << (c - b'a'))
                .count_ones()
        })
        .sum()
}

pub fn solve_part2(input: &str) -> u32 {
    input
        .split("\n\n")
        .map(|group| {
            group
                .lines()
                .map(|person| person.bytes().fold(0, |set, c| set | 1 << (c - b'a')))
                .fold(u32::MAX, |a, b| a & b)
                .count_ones()
        })
        .sum()
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
        assert_eq!(11, solve_part1(INPUT));
    }

    #[test]
    fn test_part2() {
        assert_eq!(6, solve_part2(INPUT));
    }
}
