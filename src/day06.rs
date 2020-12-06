use std::collections::HashSet;

// pub fn solve_part1(input: &str) -> usize {
//     input
//         .split("\n\n")
//         .map(|group| {
//             group
//                 .lines()
//                 .flat_map(str::chars)
//                 .collect::<HashSet<_>>()
//                 .len()
//         })
//         .sum()
// }

// A little uglier but ~10x faster than above
pub fn solve_part1(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|group| {
            let mut existence = [false; 26];
            group
                .lines()
                .flat_map(str::as_bytes)
                .map(|c| {
                    let idx = (c - b'a') as usize;
                    let existed = existence[idx];
                    existence[idx] = true;
                    !existed as usize
                })
                .sum::<usize>()
        })
        .sum()
}

pub fn solve_part2(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|group| {
            group
                .lines()
                .fold(
                    [
                        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o',
                        'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
                    ]
                    .iter()
                    .cloned()
                    .collect::<HashSet<_>>(),
                    |set, person| {
                        set.intersection(&person.chars().collect())
                            .copied()
                            .collect()
                    },
                )
                .len()
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
        assert_eq!(11, solve_part1(&INPUT));
    }

    #[test]
    fn test_part2() {
        assert_eq!(6, solve_part2(&INPUT));
    }
}
