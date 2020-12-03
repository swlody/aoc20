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

#[aoc(day3, part1)]
fn solve_part1(input: &str) -> Option<usize> {
    let line_width = input.find('\n')?;
    Some(trees_hit(input, line_width, &(3, 1)))
}

#[aoc(day3, part2)]
fn solve_part2(input: &str) -> Option<usize> {
    let line_width = input.find('\n')?;
    Some(
        [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
            .iter()
            .fold(1, |acc, slope| acc * trees_hit(input, line_width, slope)),
    )
}
