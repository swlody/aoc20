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

use chrono::{prelude::*, FixedOffset, TimeZone};
use std::time::Instant;

fn main() {
    let est = FixedOffset::west(5 * 3600).from_utc_datetime(&Utc::now().naive_utc());
    let (mut day, mut run_all) = if est.month() == 12 && est.day() <= 25 {
        (est.day(), false)
    } else {
        (0, true)
    };
    let mut args = std::env::args();
    while let Some(arg) = args.next() {
        if arg == "-d" || arg == "--day" {
            day = args.next().unwrap().parse().unwrap();
            assert!(day <= 25);
            run_all = false;
            break;
        } else if arg == "--all" {
            run_all = true;
            break;
        }
    }

    let mut total_time = std::time::Duration::new(0, 0);

    // TODO seriously write a macro for all this
    if run_all || day == 1 {
        println!("Day 1:");
        let input = include_str!("../input/day01.txt");

        let generator_start = Instant::now();
        let input = day01::input_generator(&input);
        let generator_time = generator_start.elapsed();
        println!("  Generator: {:?}", generator_time);

        let part1_start = Instant::now();
        let output_part1 = day01::solve_part1(&input);
        let part1_time = part1_start.elapsed();
        println!("  Part 1: {}\n    in {:?}", output_part1, part1_time);

        let part2_start = Instant::now();
        let output_part2 = day01::solve_part2(&input);
        let part2_time = part2_start.elapsed();
        println!("  Part 2: {}\n    in {:?}\n", output_part2, part2_time);
        total_time += generator_time + part1_time + part2_time;
    }

    if run_all || day == 2 {
        println!("Day 2:");
        let input = include_str!("../input/day02.txt");

        let generator_start = Instant::now();
        let input = day02::input_generator(&input);
        let generator_time = generator_start.elapsed();
        println!("  Generator: {:?}", generator_time);

        let part1_start = Instant::now();
        let output_part1 = day02::solve_part1(&input);
        let part1_time = part1_start.elapsed();
        println!("  Part 1: {}\n    in {:?}", output_part1, part1_time);

        let part2_start = Instant::now();
        let output_part2 = day02::solve_part2(&input);
        let part2_time = part2_start.elapsed();
        println!("  Part 2: {}\n    in {:?}\n", output_part2, part2_time);
        total_time += generator_time + part1_time + part2_time;
    }

    if run_all || day == 3 {
        println!("Day 3:");
        let input = include_str!("../input/day03.txt");

        let part1_start = Instant::now();
        let output_part1 = day03::solve_part1(&input);
        let part1_time = part1_start.elapsed();
        println!("  Part 1: {}\n    in {:?}", output_part1, part1_time);

        let part2_start = Instant::now();
        let output_part2 = day03::solve_part2(&input);
        let part2_time = part2_start.elapsed();
        println!("  Part 2: {}\n    in {:?}\n", output_part2, part2_time);
        total_time += part1_time + part2_time;
    }

    if run_all || day == 4 {
        println!("Day 4:");
        let input = include_str!("../input/day04.txt");

        let generator_start = Instant::now();
        let input = day04::input_generator(&input);
        let generator_time = generator_start.elapsed();
        println!("  Generator: {:?}", generator_time);

        let part1_start = Instant::now();
        let output_part1 = day04::solve_part1(&input);
        let part1_time = part1_start.elapsed();
        println!("  Part 1: {}\n    in {:?}", output_part1, part1_time);

        let part2_start = Instant::now();
        let output_part2 = day04::solve_part2(&input);
        let part2_time = part2_start.elapsed();
        println!("  Part 2: {}\n    in {:?}\n", output_part2, part2_time);
        total_time += generator_time + part1_time + part2_time;
    }

    if run_all || day == 5 {
        println!("Day 5:");
        let input = include_str!("../input/day05.txt");

        let part1_start = Instant::now();
        let output_part1 = day05::solve_part1(&input);
        let part1_time = part1_start.elapsed();
        println!("  Part 1: {}\n    in {:?}", output_part1, part1_time);

        let part2_start = Instant::now();
        let output_part2 = day05::solve_part2(&input);
        let part2_time = part2_start.elapsed();
        println!("  Part 2: {}\n    in {:?}\n", output_part2, part2_time);
        total_time += part1_time + part2_time;
    }

    if run_all || day == 6 {
        println!("Day 6:");
        let input = include_str!("../input/day06.txt");

        let part1_start = Instant::now();
        let output_part1 = day06::solve_part1(&input);
        let part1_time = part1_start.elapsed();
        println!("  Part 1: {}\n    in {:?}", output_part1, part1_time);

        let part2_start = Instant::now();
        let output_part2 = day06::solve_part2(&input);
        let part2_time = part2_start.elapsed();
        println!("  Part 2: {}\n    in {:?}\n", output_part2, part2_time);
        total_time += part1_time + part2_time;
    }

    if run_all || day == 7 {
        println!("Day 7:");
        let input = include_str!("../input/day07.txt");

        let generator_start = Instant::now();
        let input = day07::input_generator(&input);
        let generator_time = generator_start.elapsed();
        println!("  Generator: {:?}", generator_time);

        let part1_start = Instant::now();
        let output_part1 = day07::solve_part1(&input);
        let part1_time = part1_start.elapsed();
        println!("  Part 1: {}\n    in {:?}", output_part1, part1_time);

        let part2_start = Instant::now();
        let output_part2 = day07::solve_part2(&input);
        let part2_time = part2_start.elapsed();
        println!("  Part 2: {}\n    in {:?}\n", output_part2, part2_time);
        total_time += generator_time + part1_time + part2_time;
    }

    println!("Total time: {:?}", total_time);
}
