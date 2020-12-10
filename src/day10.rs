use fnv::FnvHashMap as HashMap;

pub fn input_generator(input: &str) -> Vec<u32> {
    let mut input: Vec<_> = input.lines().map(|line| line.parse().unwrap()).collect();
    input.sort_unstable();
    input
}

pub fn solve_part1(input: &[u32]) -> u32 {
    let (ones, threes) = input
        .windows(2)
        .fold((1, 1), |(ones, threes), w| match w[1] - w[0] {
            1 => (ones + 1, threes),
            3 => (ones, threes + 1),
            _ => (ones, threes),
        });
    ones * threes
}

pub fn solve_part2(input: &[u32]) -> u64 {
    let mut cache = HashMap::default();
    cache.insert(0, 1);
    for &joltage in input {
        let chains = joltage
            .checked_sub(1)
            .and_then(|res| cache.get(&res))
            .unwrap_or(&0)
            + joltage
                .checked_sub(2)
                .and_then(|res| cache.get(&res))
                .unwrap_or(&0)
            + joltage
                .checked_sub(3)
                .and_then(|res| cache.get(&res))
                .unwrap_or(&0);
        cache.insert(joltage, chains);
    }
    cache[input.last().unwrap()]
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT1: &str = "16
10
15
5
1
11
7
19
6
12
4";

    static INPUT2: &str = "28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3";

    #[test]
    fn test_part1() {
        let input = input_generator(INPUT1);
        assert_eq!(35, solve_part1(&input));

        let input = input_generator(INPUT2);
        assert_eq!(220, solve_part1(&input));
    }

    #[test]
    fn test_part2() {
        let input = input_generator(INPUT1);
        assert_eq!(8, solve_part2(&input));

        let input = input_generator(INPUT2);
        assert_eq!(19208, solve_part2(&input));
    }
}
