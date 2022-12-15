use std::env;

mod aoc;
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
mod day14;

fn main() {
    let first_arg = env::args().nth(1).unwrap_or(String::new());
    let all = first_arg.is_empty();

    println!("Advent of Code 2022!");

    if all || first_arg == "1" {
        println!("day 1a: {}", day01::a());
        println!("day 1b: {}", day01::b());
    }

    if all || first_arg == "2" {
        println!("day 2a: {}", day02::a());
        println!("day 2b: {}", day02::b());
    }

    if all || first_arg == "3" {
        println!("day 3a: {}", day03::a());
        println!("day 3b: {}", day03::b());
    }

    if all || first_arg == "4" {
        println!("day 4a: {}", day04::a());
        println!("day 4b: {}", day04::b());
    }

    if all || first_arg == "5" {
        println!("day 5a: {}", day05::a());
        println!("day 5b: {}", day05::b());
    }

    if all || first_arg == "6" {
        println!("day 6a: {}", day06::a());
        println!("day 6b: {}", day06::b());
    }

    if all || first_arg == "7" {
        println!("day 7a: {}", day07::a());
        println!("day 7b: {}", day07::b());
    }

    if all || first_arg == "8" {
        println!("day 8a: {}", day08::a());
        println!("day 8b: {}", day08::b());
    }

    if all || first_arg == "9" {
        println!("day 9a: {}", day09::a());
        println!("day 9b: {}", day09::b());
    }

    if all || first_arg == "10" {
        println!("day 10a: {}", day10::a());
        println!("day 10b: {}", day10::b());
    }

    if all || first_arg == "11" {
        println!("day 11a: {}", day11::a());
        println!("day 11b: {}", day11::b());
    }

    if all || first_arg == "12" {
        println!("day 12a: {}", day12::a());
        println!("day 12b: {}", day12::b());
    }

    if all || first_arg == "13" {
        println!("day 13a: {}", day13::a());
        println!("day 13b: {}", day13::b());
    }

    if all || first_arg == "14" {
        println!("day 14a: {}", day14::a());
    }
}
