use std::collections::BTreeSet;

pub fn input_generator(input: &str) -> BTreeSet<u32> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

fn get_pair(set: &BTreeSet<u32>, sum: u32) -> Option<(u32, u32)> {
    for &num in set.iter() {
        if let Some(&result) = set.get(&(sum.checked_sub(num)?)) {
            return Some((num, result));
        }
    }
    None
}

pub fn solve_part1(set: &BTreeSet<u32>) -> u32 {
    let (a, b) = get_pair(set, 2020).unwrap();
    a * b
}

pub fn solve_part2(set: &BTreeSet<u32>) -> u32 {
    for &num in set.iter() {
        if let Some((a, b)) = get_pair(set, 2020 - num) {
            return a * b * num;
        }
    }
    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = "1721
979
366
299
675
1456";

    #[test]
    fn test_part1() {
        let set = input_generator(INPUT);
        assert_eq!(514_579, solve_part1(&set));
    }

    #[test]
    fn test_part2() {
        let set = input_generator(INPUT);
        assert_eq!(241_861_950, solve_part2(&set));
    }
}
