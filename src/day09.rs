pub fn input_generator(input: &str) -> Vec<u64> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

fn solve_part1_with_preamble_length(numbers: &[u64], preamble_length: usize) -> u64 {
    let offset = numbers
        .windows(preamble_length)
        .enumerate()
        .find(|&(offset, window)| {
            let number = numbers[preamble_length + offset];
            for &other_number in window.iter() {
                let result = number
                    .checked_sub(other_number)
                    .unwrap_or_else(|| other_number - number);

                if window.contains(&result) {
                    return false;
                }
            }
            true
        })
        .expect("No answer found")
        .0;

    numbers[preamble_length + offset]
}

use std::cmp::Ordering;

fn solve_part2_from_part1(numbers: &[u64], part1_result: u64) -> u64 {
    for (start_point, &start_number) in numbers.iter().enumerate() {
        let mut result = start_number;
        let (mut min, mut max) = (result, result);
        for &next_number in numbers[start_point + 1..numbers.len() - start_point].iter() {
            match (result + next_number).cmp(&part1_result) {
                Ordering::Greater => break,
                Ordering::Equal => return min + max,
                Ordering::Less => {
                    min = std::cmp::min(min, next_number);
                    max = std::cmp::max(max, next_number);
                    result += next_number;
                }
            }
        }
    }
    panic!("No answer found")
}

pub fn solve_both_parts(input: &[u64]) -> (u64, u64) {
    let part1_result = solve_part1_with_preamble_length(input, 25);
    let part2_result = solve_part2_from_part1(input, part1_result);
    (part1_result, part2_result)
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = "35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576";

    #[test]
    fn test_both_parts() {
        let input = input_generator(INPUT);
        let part1_result = solve_part1_with_preamble_length(&input, 5);
        assert_eq!(127, part1_result);
        let part2_result = solve_part2_from_part1(&input, part1_result);
        assert_eq!(62, part2_result);
    }
}
