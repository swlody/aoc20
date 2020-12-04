#![feature(str_split_once)]
#![feature(array_map)]
#![feature(or_patterns)]

#[macro_use]
extern crate aoc_runner_derive;

pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;

aoc_runner_derive::aoc_lib! { year = 2020 }
