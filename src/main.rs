#![feature(array_map)]
#![feature(destructuring_assignment)]
#![feature(iterator_fold_self)]
#![feature(map_first_last)]
#![feature(or_patterns)]
#![feature(str_split_once)]

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;

aoc_main::main! {
    year 2020;
    day01 : input_generator => solve_part1, solve_part2;
    day02 : input_generator => solve_part1, solve_part2;
    day03                   => solve_part1, solve_part2;
    day04 : input_generator => solve_part1, solve_part2;
    day05                   => solve_part1, solve_part2;
    day06                   => solve_part1, solve_part2;
    day07 : input_generator => solve_part1, solve_part2;
    day08 : input_generator => solve_part1, solve_part2;
    day09 : input_generator => solve_both_parts;
    day10 : input_generator => solve_part1, solve_part2;
    day11 : input_generator => solve_part1, solve_part2;
    day12 : input_generator => solve_part1, solve_part2;
    day13                   => solve_part1;
}
