#![feature(str_split_once)]
#![feature(array_map)]
#![feature(or_patterns)]
#![feature(iterator_fold_self)]
#![feature(map_first_last)]

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;

aoc::main! {
    year 2020;
    day01 : input_generator => solve_part1, solve_part2;
    day02 : input_generator => solve_part1, solve_part2;
    day03                   => solve_part1, solve_part2;
    day04 : input_generator => solve_part1, solve_part2;
    day05                   => solve_part1, solve_part2;
    day06                   => solve_part1, solve_part2;
    day07 : input_generator => solve_part1, solve_part2;
}
