pub fn input_generator(input: &str) -> Vec<u64> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

fn solve_part1_with_preamble_length(preamble_length: usize, numbers: &[u64]) -> u64 {
    let mut found = 0;
    'outer: for (idx, &number) in numbers.iter().enumerate().skip(preamble_length) {
        let window = &numbers[idx - preamble_length..idx];
        for &other_number in window.iter() {
            let result = number
                .checked_sub(other_number)
                .unwrap_or_else(|| other_number - number);

            if number != other_number && window.contains(&result) {
                continue 'outer;
            }
        }

        found = number;
        break;
    }
    found
}

use std::cmp::Ordering;

fn solve_part2_from_part1(numbers: &[u64], part1_result: u64) -> u64 {
    'outer: for start_point in 0..numbers.len() {
        let mut result = 0;
        let mut offset = 0;
        loop {
            let next_number = numbers[start_point + offset];
            match (result + next_number).cmp(&part1_result) {
                Ordering::Equal if offset > 0 => {
                    let range = &numbers[start_point..start_point + offset];
                    let low_value = range.iter().min().unwrap();
                    let high_value = range.iter().max().unwrap();
                    return low_value + high_value;
                }
                Ordering::Less => {
                    offset += 1;
                    result += next_number;
                }
                _ => continue 'outer,
            }
        }
    }
    panic!("No answer found")
}

pub struct Answer(u64, u64);

use std::fmt;
impl fmt::Display for Answer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Part 1: {}, Part 2: {}", self.0, self.1)
    }
}

pub fn solve_both_parts(input: &[u64]) -> Answer {
    let part1_result = solve_part1_with_preamble_length(25, input);
    let part2_result = solve_part2_from_part1(input, part1_result);
    Answer(part1_result, part2_result)
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
        let part1_result = solve_part1_with_preamble_length(5, &input);
        assert_eq!(127, part1_result);
        let part2_result = solve_part2_from_part1(&input, part1_result);
        assert_eq!(62, part2_result);
    }
}
