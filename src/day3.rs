use aoc_runner_derive::aoc;

fn solve(input: &str, slopes: &[(usize, usize)]) -> usize {
    let line_width = input.find('\n').unwrap();
    slopes.iter().fold(1, |acc, &(run, fall)| {
        acc * input
            .lines()
            .step_by(fall)
            .zip((0..line_width).cycle().step_by(run))
            .skip(1)
            .filter(|&(line, idx)| line.get(idx..=idx).unwrap() == "#")
            .count()
    })
}

#[aoc(day3, part1)]
fn solve_part1(input: &str) -> usize {
    solve(&input, &[(3, 1)])
}

#[aoc(day3, part2)]
fn solve_part2(input: &str) -> usize {
    solve(&input, &[(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)])
}
