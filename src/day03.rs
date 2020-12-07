fn trees_hit(input: &str, line_width: usize, slope: &(usize, usize)) -> usize {
    let &(x_step, y_step) = slope;
    input
        .lines()
        .step_by(y_step)
        .zip((0..line_width).cycle().step_by(x_step))
        .skip(1)
        .filter(|&(line, idx)| line.as_bytes()[idx] as char == '#')
        .count()
}

pub fn solve_part1(input: &str) -> usize {
    let line_width = input.find('\n').unwrap();
    trees_hit(input, line_width, &(3, 1))
}

pub fn solve_part2(input: &str) -> usize {
    let line_width = input.find('\n').unwrap();
    [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
        .map(|slope| trees_hit(input, line_width, &slope))
        .iter()
        .product()
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = "..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#";

    #[test]
    fn test_part1() {
        assert_eq!(7, solve_part1(INPUT));
    }

    #[test]
    fn test_part2() {
        assert_eq!(336, solve_part2(INPUT));
    }
}
